use crate::errors::*;

#[macro_export]
macro_rules! require_valid_status {
    ($status:expr) => {
        require!($status != 1 && $status != 2, MemberError::AccountSuspended);
    };
}
