
pub enum PlayerState {
    Idle,
    Walking,
    Running,
    /// This is when the spider have a web connected, 
    /// not grounded and at D > d (with D = Player_pos - web_pos and d = web length)
    Hanging,
    /// Execute after jump
    Jumping,
    /// Execute right when player change from airborne to grounded
    Landing,
    /// When not grounded and not qualified for hanging, this will be the state in use
    Floating
}