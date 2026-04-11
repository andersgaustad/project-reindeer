use clap::Parser;


#[derive(Debug, Parser)]
#[command(version, about, propagate_version = true, long_about = None)]
pub struct ClapParser {
    /// Version string (x.y.z)
    #[arg(short, long)]
    pub version : String,
}
