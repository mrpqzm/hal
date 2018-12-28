use std::collections::HashMap;

use bitcoin::util::psbt;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct PsbtGlobalInfo {
	pub unsigned_tx: ::tx::TransactionInfo,
}

impl PsbtGlobalInfo {
	pub fn create(global: &psbt::Global, testnet: bool) -> PsbtGlobalInfo {
		PsbtGlobalInfo {
			unsigned_tx: ::tx::TransactionInfo::create(&global.unsigned_tx, testnet),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct HDPathInfo {
	pub master_fingerprint: ::HexBytes,
	pub path: String,
}

fn string_sighashtype(sht: bitcoin::SigHashType) -> String {
	use bitcoin::SigHashType::*;
	match sht {
		All => "ALL",
		None => "NONE",
		Single => "SINGLE",
		AllPlusAnyoneCanPay => "ALL|ANYONECANPAY",
		NonePlusAnyoneCanPay => "NONE|ANYONECANPAY",
		SinglePlusAnyoneCanPay => "SINGLE|ANYONECANPAY",
	}
	.to_owned()
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct PsbtInputInfo {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub non_witness_utxo: Option<::tx::TransactionInfo>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub witness_utxo: Option<::tx::OutputInfo>,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub partial_sigs: HashMap<::HexBytes, ::HexBytes>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sighash_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redeem_script: Option<::tx::OutputScriptInfo>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub witness_script: Option<::tx::OutputScriptInfo>,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub hd_keypaths: HashMap<::HexBytes, HDPathInfo>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub final_script_sig: Option<::tx::InputScriptInfo>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub final_script_witness: Option<Vec<::HexBytes>>,
}

fn string_derivation_path(path: &[bitcoin::util::bip32::ChildNumber]) -> String {
	use std::fmt::Write;
	let mut buf = String::new();
	buf.write_str("m").unwrap();
	for cn in path.iter() {
		buf.write_fmt(format_args!("/{}", cn)).unwrap();
	}
	buf
}

impl PsbtInputInfo {
	pub fn create(input: &psbt::Input, testnet: bool) -> PsbtInputInfo {
		PsbtInputInfo {
			non_witness_utxo: input
				.non_witness_utxo
				.as_ref()
				.map(|u| ::tx::TransactionInfo::create(&u, testnet)),
			witness_utxo: input.witness_utxo.as_ref().map(|u| ::tx::OutputInfo::create(&u, testnet)),
			partial_sigs: {
				let mut partial_sigs = HashMap::new();
				for (key, value) in input.partial_sigs.iter() {
					partial_sigs.insert(key.serialize()[..].into(), value.clone().into());
				}
				partial_sigs
			},
			sighash_type: input.sighash_type.map(string_sighashtype),
			redeem_script: input
				.redeem_script
				.as_ref()
				.map(|s| ::tx::OutputScriptInfo::create(&s, testnet)),
			witness_script: input
				.witness_script
				.as_ref()
				.map(|s| ::tx::OutputScriptInfo::create(&s, testnet)),
			hd_keypaths: {
				let mut hd_keypaths = HashMap::new();
				for (key, value) in input.hd_keypaths.iter() {
					hd_keypaths.insert(
						key.serialize()[..].into(),
						HDPathInfo {
							master_fingerprint: value.0[..].into(),
							path: string_derivation_path(&value.1),
						},
					);
				}
				hd_keypaths
			},
			final_script_sig: input
				.final_script_sig
				.as_ref()
				.map(|s| ::tx::InputScriptInfo::create(&s)),
			final_script_witness: input
				.final_script_witness
				.as_ref()
				.map(|w| w.iter().map(|p| p.clone().into()).collect()),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct PsbtOutputInfo {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redeem_script: Option<::tx::OutputScriptInfo>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub witness_script: Option<::tx::OutputScriptInfo>,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub hd_keypaths: HashMap<::HexBytes, HDPathInfo>,
}

impl PsbtOutputInfo {
	pub fn create(output: &psbt::Output, testnet: bool) -> PsbtOutputInfo {
		PsbtOutputInfo {
			redeem_script: output
				.redeem_script
				.as_ref()
				.map(|s| ::tx::OutputScriptInfo::create(&s, testnet)),
			witness_script: output
				.witness_script
				.as_ref()
				.map(|s| ::tx::OutputScriptInfo::create(&s, testnet)),
			hd_keypaths: {
				let mut hd_keypaths = HashMap::new();
				for (key, value) in output.hd_keypaths.iter() {
					hd_keypaths.insert(
						key.serialize()[..].into(),
						HDPathInfo {
							master_fingerprint: value.0[..].into(),
							path: string_derivation_path(&value.1),
						},
					);
				}
				hd_keypaths
			},
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct PsbtInfo {
	pub global: PsbtGlobalInfo,
	pub inputs: Vec<PsbtInputInfo>,
	pub outputs: Vec<PsbtOutputInfo>,
}

impl PsbtInfo {
	pub fn create(psbt: &psbt::PartiallySignedTransaction, testnet: bool) -> PsbtInfo {
		PsbtInfo {
			global: PsbtGlobalInfo::create(&psbt.global, testnet),
			inputs: psbt.inputs.iter().map(|i| PsbtInputInfo::create(&i, testnet)).collect(),
			outputs: psbt.outputs.iter().map(|i| PsbtOutputInfo::create(&i, testnet)).collect(),
		}
	}
}
