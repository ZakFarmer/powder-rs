use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub struct Config {
    /// Whether or not barriers at the screen boundary are enabled
    /// (it affects performance a lot atm when there are a lot of particles on the screen, so might be best to leave this off)
    pub barriers_on: bool,
}

impl Config {
    fn new() -> Config {
        Config { barriers_on: true }
    }
}
