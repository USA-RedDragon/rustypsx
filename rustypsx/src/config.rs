use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RawConfig {
    /// Scale factor for GUI
    #[arg(short, long, default_value_t = 1)]
    scale: u8,
}

pub struct CleanConfig {
    // GUI scale factor
    pub scale: u8,
}

impl RawConfig {
    pub fn clean(self) -> CleanConfig {
        CleanConfig {
            scale: self.scale,
        }
    }
}
