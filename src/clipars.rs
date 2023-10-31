use clap::Parser;

/// DTMF Decoder.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// Length of the frame in samples
    #[arg(short, long, default_value_t = 512)]
    pub frame_len: i32,
    /// Output device index. For default one, leave as blank.
    #[arg(short, long, default_value_t = -1)]
    pub device: i32,
    /// list devices
    #[arg(short, long)]
    pub list_devices: bool,
}
