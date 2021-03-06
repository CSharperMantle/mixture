use clap::Parser;

const MIXES_BANNER: &'static str = r#"
 __  __ _______   __         
|  \/  |_   _\ \ / /         
| \  / | | |  \ V / ___  ___ 
| |\/| | | |   > < / _ \/ __|
| |  | |_| |_ / . \  __/\__ \
|_|  |_|_____/_/ \_\___||___/

MIXes: Frontend for MIX emulator/simulator.
"#;

#[derive(Parser)]
#[clap(
    name = "mixes",
    author,
    version,
    about = MIXES_BANNER,
)]
struct Args {
    /// Input MIX bytecode file
    file: String,

    /// Suppress banner display after startup
    #[clap(long)]
    no_banner: bool,
}

fn main() {
    let args = Args::parse();
    if !args.no_banner {
        println!("{}", MIXES_BANNER);
    }
    let path = std::path::Path::new(&args.file);
    let content = String::from_utf8_lossy(&std::fs::read(path).unwrap()).to_string();
    println!("{:?}", content);
}
