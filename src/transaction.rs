use crate::{CodecError, codec::ParseEnumError};

pub struct Transaction {
    pub tx_id: i64,
    pub tx_type: TransactionType,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub amount: i64,
    pub timestamp: i64,
    pub status: TransactionStatus,
    pub description: String
}

#[derive(Copy, Clone)]
pub enum TransactionType {
    Deposit,
    Transfer,
    Withdraw
}

#[derive(Copy, Clone)]
pub enum TransactionStatus {
    Success,
    Failure,
    Pending
}

impl Transaction {
    pub const TX_ID_NAME: &str = "TX_ID";
    pub const TX_TYPE_NAME: &str = "TX_TYPE";
    pub const FROM_USER_ID_NAME: &str = "FROM_USER_ID";
    pub const TO_USER_ID_NAME: &str = "TO_USER_ID";
    pub const AMOUNT_NAME: &str = "AMOUNT";
    pub const TIMESTAMP_NAME: &str = "TIMESTAMP";
    pub const STATUS_NAME: &str = "STATUS";
    pub const DESCRIPTION_NAME: &str = "DESCRIPTION";
}

#[derive(Default)]
pub struct TransactionBuilder {
    pub tx_id: Option<i64>,
    pub tx_type: Option<TransactionType>,
    pub from_user_id: Option<i64>,
    pub to_user_id: Option<i64>,
    pub amount: Option<i64>,
    pub timestamp: Option<i64>,
    pub status: Option<TransactionStatus>,
    pub description: Option<String>
}

impl TransactionBuilder {
    pub fn build(self) -> Result<Transaction, CodecError> {
        Ok(Transaction {
            tx_id: self.tx_id.ok_or(CodecError::MissingField(Transaction::TX_ID_NAME.into()))?,
            tx_type: self.tx_type.ok_or(CodecError::MissingField(Transaction::TX_TYPE_NAME.into()))?,
            from_user_id: self.from_user_id.ok_or(CodecError::MissingField(Transaction::FROM_USER_ID_NAME.into()))?,
            to_user_id: self.to_user_id.ok_or(CodecError::MissingField(Transaction::TO_USER_ID_NAME.into()))?,
            amount: self.amount.ok_or(CodecError::MissingField(Transaction::AMOUNT_NAME.into()))?,
            timestamp: self.timestamp.ok_or(CodecError::MissingField(Transaction::TIMESTAMP_NAME.into()))?,
            status: self.status.ok_or(CodecError::MissingField(Transaction::STATUS_NAME.into()))?,
            description: self.description.ok_or(CodecError::MissingField(Transaction::DESCRIPTION_NAME.into()))?,
        })
    }
}

impl std::str::FromStr for TransactionType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "TRANSFER" => Ok(TransactionType::Transfer),
            "WITHDRAWAL" => Ok(TransactionType::Withdraw),
            _ => Err(ParseEnumError)
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Err(ParseEnumError)
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\n\
                AMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: {}",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            self.description
        )
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TransactionType::Deposit => "DEPOSIT",
            TransactionType::Transfer => "TRANSFER",
            TransactionType::Withdraw => "WITHDRAWAL"
        })
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TransactionStatus::Success => "SUCCESS",
            TransactionStatus::Failure => "FAILURE",
            TransactionStatus::Pending => "PENDING"
        })
    }
}
