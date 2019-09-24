use structopt::StructOpt;
use steamid_ng::SteamID;

#[derive(StructOpt, Debug)]
struct Cli {
    id: String,
}

fn main() {
    let args = Cli::from_args();

    let id: SteamID;

    if args.id.contains("STEAM") {
       id = SteamID::from_steam2(&args.id).unwrap();
    } else if args.id.contains("[") {
        id = SteamID::from_steam3(&args.id).unwrap();
    } else if args.id.parse::<u64>().is_ok() {
        id = SteamID::from(args.id.parse::<u64>().unwrap());
    } else {
        return
    }

    println!("SteamID64: {}", u64::from(id));
    println!("SteamID2: {}", id.steam2());
    println!("SteamID3: {}", id.steam3());
}
