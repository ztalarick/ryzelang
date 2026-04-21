use ryzelang_core::interpreter::Interpreter;
use clap::Parser;
use std::fs;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .ryze file
    file: String,

    /// Print the final state of the stacks
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let source = fs::read_to_string(&args.file)?;
    let mut interpreter = Interpreter::new(source);
    
    if let Err(e) = interpreter.run() {
        eprintln!("\nError: {}", e);
        eprintln!("\n{}", interpreter.runtime.format_stacks());
        std::process::exit(1);
    }

    if args.debug {
        println!("\n{}", interpreter.runtime.format_stacks());
    }

    Ok(())
}
