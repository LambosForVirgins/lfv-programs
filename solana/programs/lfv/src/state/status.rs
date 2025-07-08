use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum AccountStatus {
    Pending,   // Initialised member
    Active,    // Activated member account
    Paused,    // Temporary self-exclusion
    Excluded,  // Self exclusion from member benefits
    Suspended, // System exclusion from member benefits
}

impl AccountStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => AccountStatus::Pending,
            1 => AccountStatus::Active,
            2 => AccountStatus::Paused,
            3 => AccountStatus::Excluded,
            4 => AccountStatus::Suspended,
            _ => panic!("Invalid account status"),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            AccountStatus::Pending => 0,
            AccountStatus::Active => 1,
            AccountStatus::Paused => 2,
            AccountStatus::Excluded => 3,
            AccountStatus::Suspended => 4,
        }
    }

    pub fn equals(self, value: u8) -> bool {
        self == Self::from_u8(value)
    }

    pub fn not(self, value: u8) -> bool {
        !self.equals(value)
    }
}
