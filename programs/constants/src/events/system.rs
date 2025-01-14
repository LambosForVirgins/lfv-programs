#[event]
pub struct EmergencyPauseEvent {
    pub state: Pubkey,
}

#[event]
pub struct ResumeEvent {
    pub state: Pubkey,
}
