//very nice lib that reads arguments, builds on clap
use structopt::StructOpt;

// Here we formalize the type of input we expect when calling the program
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

//cargo run -- pattern path(test.txt)
fn main() {
    let args = Cli::from_args();
    
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Oh noes: {}", error); }
    };
    println!("File content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
