use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Transaction {
    tx_id: i64,
    tx_type: TransactionType,
    from_user_id: i64,
    to_user_id: i64,
    amount: i64,
    timestamp: i64,
    status: TransactionStatus,
    description: String
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Deposit,
    Transfer,
    Withdrawal
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
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

    pub fn csv_header() -> String {
        format!("{},{},{},{},{},{},{},{}",
            Transaction::TX_ID_NAME,
            Transaction::TX_TYPE_NAME,
            Transaction::FROM_USER_ID_NAME,
            Transaction::TO_USER_ID_NAME,
            Transaction::AMOUNT_NAME,
            Transaction::TIMESTAMP_NAME,
            Transaction::STATUS_NAME,
            Transaction::DESCRIPTION_NAME,
        )
    }

    pub fn new(tx_id: i64, tx_type: TransactionType, from_user_id: i64, to_user_id: i64,
               amount: i64, timestamp: i64, status: TransactionStatus, description: String) -> Self {
        Transaction { tx_id, tx_type, from_user_id, to_user_id, amount, timestamp, status, description }
    }

    pub fn get_tx_id(&self) -> i64 {
        self.tx_id
    }

    pub fn get_tx_type(&self) -> TransactionType {
        self.tx_type
    }

    pub fn get_from_user_id(&self) -> i64 {
        self.from_user_id
    }

    pub fn get_to_user_id(&self) -> i64 {
        self.to_user_id
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_status(&self) -> TransactionStatus {
        self.status
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}': '{}', '{}': '{}', '{}': '{}', '{}': '{}', \
                '{}': '{}', '{}': '{}', '{}': '{}', '{}': '{}'",
                Transaction::TX_ID_NAME, self.tx_id,
                Transaction::TX_TYPE_NAME, self.tx_type,
                Transaction::FROM_USER_ID_NAME, self.from_user_id,
                Transaction::TO_USER_ID_NAME, self.to_user_id,
                Transaction::AMOUNT_NAME, self.amount,
                Transaction::TIMESTAMP_NAME, self.timestamp,
                Transaction::STATUS_NAME, self.status,
                Transaction::DESCRIPTION_NAME, self.description
        )
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TransactionType::Deposit => "DEPOSIT",
            TransactionType::Transfer => "TRANSFER",
            TransactionType::Withdrawal => "WITHDRAWAL"
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
