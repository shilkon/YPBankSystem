use crate::codec::ParseTextError;

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

pub enum TransactionType {
    Deposit,
    Transfer,
    Withdraw
}

pub enum TransactionStatus {
    Success,
    Failure,
    Pending
}

impl std::str::FromStr for TransactionType {
    type Err = ParseTextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "TRANSFER" => Ok(TransactionType::Transfer),
            "WITHDRAWAL" => Ok(TransactionType::Withdraw),
            _ => Err(Self::Err::new("TX_TYPE", s ))
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = ParseTextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Err(Self::Err::new("STATUS", s ))
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction:\n \
                TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID:{}\n \
                Amount: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: {}",
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
