use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), /*Box<dyn std::error::Error>*/ /*CustomError*/ ExitFailure> {
    let args: Cli = Cli::from_args();
    //let result = std::fs::read_to_string(&args.path);
    //let content = match result {
    //    Ok(content) => { content },
    //    Err(error)  => { return Err(error.into()); }
    //};

    //let content = std::fs::read_to_string(&args.path)?;

    //let content = std::fs::read_to_string(&args.path)
    //    .map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.to_str().unwrap(), err)))?;

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", &args.path.to_str().unwrap()))?;

    println!("file content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
