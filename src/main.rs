use clap::Parser;
use moseiik::{compute_mosaic, Options};

fn main() {
    let args = Options::parse();
    compute_mosaic(args);
}
