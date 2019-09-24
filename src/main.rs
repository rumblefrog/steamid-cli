use structopt::StructOpt;

mod steamid;

#[derive(StructOpt, Debug)]
struct Cli {
    id: String,
}

fn main() {
    let args = Cli::from_args();

    println!("{:?}", args);
}
