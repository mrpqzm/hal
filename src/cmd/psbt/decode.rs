
use bitcoin::util::psbt;
use clap;
use hal;

pub fn subcommand<'a>() -> clap::App<'a, 'a> {
	clap::SubCommand::with_name("decode")
		.about("decode a PSBT to JSON")
		.arg(
			clap::Arg::with_name("psbt")
				.help("the PSBT file or raw PSBT in hex")
				.takes_value(true)
				.required(true),
		)
		.arg(
			// This influences the addresses we print.
			clap::Arg::with_name("testnet")
				.long("testnet")
				.help("for testnet transaction")
				.takes_value(true)
				.required(false),
		)
}

pub fn execute<'a>(matches: &clap::ArgMatches<'a>) {
	let raw_psbt = super::file_or_raw(matches.value_of("psbt").unwrap());

	let psbt: psbt::PartiallySignedTransaction =
		bitcoin::consensus::deserialize(&raw_psbt).expect("invalid PSBT");

	let info = hal::psbt::PsbtInfo::create(&psbt, matches.is_present("testnet"));
	serde_json::to_writer_pretty(::std::io::stdout(), &info).unwrap();
}
