use anyhow::Result;
use clap::Parser;
use vlc::vlc;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = vlc::Config { 
        hardware_acceleration: !cli.no_hw_accel, 
        ..Default::default() 
    };
    let mut component = vlc::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-vlc ready");
    Ok(())
}
