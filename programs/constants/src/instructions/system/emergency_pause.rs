use anchor_lang::prelude::*;

use crate::{
    error::SystemError,
    events::admin::{EmergencyPauseEvent, ResumeEvent},
    events::{EmergencyPauseEvent, ResumeEvent},
    State,
};

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        has_one = authority @ SystemError::InvalidPauseAuthority
    )]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

impl<'info> EmergencyPause<'info> {
    pub fn pause(&mut self) -> Result<()> {
        require!(!self.state.paused, SystemError::AlreadyPaused);
        self.state.paused = true;
        emit!(EmergencyPauseEvent {
            state: self.state.key(),
        });

        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        require!(self.state.paused, SystemError::NotPaused);
        self.state.paused = false;
        emit!(ResumeEvent {
            state: self.state.key(),
        });
        Ok(())
    }
}
