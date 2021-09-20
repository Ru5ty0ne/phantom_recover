use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use solana_sdk::signer::keypair::{read_keypair_file, write_keypair_file, Keypair};

fn main() {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("show_private_key")
                .about("Showing private key of your keypair file")
                .arg(
                    Arg::with_name("key_path")
                        .short("k")
                        .long("key_path")
                        .required(true)
                        .takes_value(true)
                        .help("Path to keypair"),
                ),
        )
        .subcommand(
            SubCommand::with_name("restore")
                .about("Restoring keypair file from private key")
                .arg(
                    Arg::with_name("private_key")
                        .short("k")
                        .long("private_key")
                        .required(true)
                        .takes_value(true)
                        .help("Your private key from Phatom wallet"),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .required(true)
                        .takes_value(true)
                        .help("Output .json file"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("show_private_key") {
        let key_path = matches.value_of("key_path").unwrap();
        let wallet_keypair = read_keypair_file(key_path).unwrap();
        println!("Your private key: {}", wallet_keypair.to_base58_string());
    }
    if let Some(matches) = matches.subcommand_matches("restore") {
        let private_key = matches.value_of("private_key").unwrap();
        let output = matches.value_of("output").unwrap();
        let raw_keypair = Keypair::from_base58_string(private_key);
        write_keypair_file(&raw_keypair, output).unwrap();
    }
}
