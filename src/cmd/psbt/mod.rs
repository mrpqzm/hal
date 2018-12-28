use std::fs::File;
use std::io::Read;

use clap;

mod decode;
mod merge;

pub fn subcommand<'a>() -> clap::App<'a, 'a> {
	clap::SubCommand::with_name("psbt")
		.about("partially signed Bitcoin transactions")
		.setting(clap::AppSettings::SubcommandRequiredElseHelp)
		.setting(clap::AppSettings::DisableHelpSubcommand)
		.subcommand(decode::subcommand())
		.subcommand(merge::subcommand())
}

pub fn execute<'a>(matches: &clap::ArgMatches<'a>) {
	match matches.subcommand() {
		("decode", Some(ref m)) => decode::execute(&m),
		("merge", Some(ref m)) => merge::execute(&m),
		(c, _) => println!("command {} unknown", c),
	};
}

pub fn file_or_raw(flag: &str) -> Vec<u8> {
	match hex::decode(&flag) {
		Ok(raw) => raw,
		Err(_) => match File::open(&flag) {
			Ok(mut file) => {
				let mut buf = Vec::new();
				file.read_to_end(&mut buf).expect("error reading file");
				buf
			},
			Err(_) => panic!("Can't load PSBT: invalid hex or unknown file"),
		}
	}
}
