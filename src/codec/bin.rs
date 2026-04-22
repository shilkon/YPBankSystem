use std::io::Cursor;

use binrw::{BinWrite, binrw};
use binrw::BinReaderExt;

use super::{
    Transaction, TransactionType, TransactionStatus,
    TransactionWriter, TransactionReader,
    CodecError,
};

pub struct BinFormat;

impl BinFormat {
    const MAGIC: u32 = 0x5950424E; //'YPBN'
}

#[binrw]
#[derive(Debug)]
#[brw(big)]
struct BinTransaction {
    tx_id: i64,
    tx_type: u8,
    from_user_id: i64,
    to_user_id: i64,
    amount: i64,
    timestamp: i64,
    status: u8,

    #[br(temp)]
    #[bw(calc = description.len() as u32)]
    desc_len: u32,

    #[br(count = desc_len)]
    #[bw(map = |v: &Vec<u8>| v.as_slice())]
    description: Vec<u8>
}

impl From<&Transaction> for BinTransaction {
    fn from(tx: &Transaction) -> Self {
        let description = tx.get_description();
        BinTransaction {
            tx_id: tx.get_tx_id(),
            tx_type: tx.get_tx_type() as u8,
            from_user_id: tx.get_from_user_id(),
            to_user_id: tx.get_to_user_id(),
            amount: tx.get_amount(),
            timestamp: tx.get_timestamp(),
            status: tx.get_status() as u8,
            description: description.into_bytes()
        }
    }
}

impl TryFrom<BinTransaction> for Transaction {
    type Error = CodecError;
    
    fn try_from(tx: BinTransaction) -> Result<Self, Self::Error> {
        Ok(Transaction::new(tx.tx_id, tx.tx_type.try_into()?, tx.from_user_id, tx.to_user_id,
            tx.amount, tx.timestamp, tx.status.try_into()?, String::from_utf8(tx.description)?))
    }
}

impl TryFrom<u8> for TransactionType {
    type Error = CodecError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransactionType::Deposit),
            1 => Ok(TransactionType::Transfer),
            2 => Ok(TransactionType::Withdrawal),
            _ => Err(CodecError::Format(format!("Invalid binary TransactionType: {value}")))
        }
    }
}

impl TryFrom<u8> for TransactionStatus {
    type Error = CodecError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransactionStatus::Success),
            1 => Ok(TransactionStatus::Failure),
            2 => Ok(TransactionStatus::Pending),
            _ => Err(CodecError::Format(format!("Invalid binary TransactionStatus: {value}")))
        }
    }
}

impl TransactionWriter for BinFormat {
    fn write_record<W: std::io::Write>(&self, w: &mut W, tx: &Transaction) -> Result<(), CodecError> {
        let bin_tx: BinTransaction = tx.into();

        let mut body = Cursor::new(Vec::new());
        bin_tx.write_be(&mut body)?;
        let body = body.into_inner();

        w.write_all(&Self::MAGIC.to_be_bytes())?;
        w.write_all(&(body.len() as u32).to_be_bytes())?;
        w.write_all(&body)?;

        Ok(())
    }
}

impl TransactionReader for BinFormat {
    fn read_next<R: std::io::BufRead>(&self, r: &mut R, pos: &mut usize) -> Result<Option<Transaction>, CodecError> {
        let buf_check = r.fill_buf()?;
        if buf_check.is_empty() {
            return Ok(None)
        }

        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        *pos += 4;
        let magic = u32::from_be_bytes(buf);
        if magic != Self::MAGIC {
            return Err(CodecError::Format(format!("MAGIC is {}, but must be {}", magic, Self::MAGIC)));
        }

        r.read_exact(&mut buf)?;
        *pos += 4;
        let record_size = u32::from_be_bytes(buf);

        let mut buf = vec![0u8; record_size as usize];
        r.read_exact(&mut buf)?;
        *pos += record_size as usize;

        let mut cursor = Cursor::new(buf);
        let bin_tx: BinTransaction = cursor.read_be()?;
        Ok(Some(bin_tx.try_into()?))
    }
}
