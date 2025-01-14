use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum MemberTier {
    Pending,
    Virgin,
    Super,
    Mega,
    Giga,
}

impl MemberTier {
    pub fn to_info(value: u8) -> Result<Self> {
        match value {
            0 => Ok(MemberTier::Pending),
            1 => Ok(MemberTier::Virgin),
            2 => Ok(MemberTier::Super),
            3 => Ok(MemberTier::Mega),
            4 => Ok(MemberTier::Giga),
            _ => panic!("Invalid membership tier"),
        }
    }

    pub fn from_tier(value: MemberTier) -> u8 {
        match value {
            MemberTier::Pending => 0,
            MemberTier::Virgin => 1,
            MemberTier::Super => 2,
            MemberTier::Mega => 3,
            MemberTier::Giga => 4,
        }
    }

    pub fn required_tokens(&self) -> u64 {
        match self {
            MemberTier::Pending => 1,
            MemberTier::Virgin => 3_000_000_000_000, // 3000 $7.50 USD @ 1.5MCAP
            MemberTier::Super => 100_000_000_000_000, // 100K $250 USD
            MemberTier::Mega => 2_000_000_000_000_000, // 2M $5000 USD
            MemberTier::Giga => 5_000_000_000_000_000, // 5M $12,000 USD
        }
    }

    pub fn get_tier(lamports: u64) -> MemberTier {
        if lamports >= MemberTier::Giga.required_tokens() {
            MemberTier::Giga
        } else if lamports >= MemberTier::Mega.required_tokens() {
            MemberTier::Mega
        } else if lamports >= MemberTier::Super.required_tokens() {
            MemberTier::Super
        } else if lamports >= MemberTier::Virgin.required_tokens() {
            MemberTier::Virgin
        } else {
            MemberTier::Pending
        }
    }
}
