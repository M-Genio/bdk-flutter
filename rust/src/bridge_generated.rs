#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.63.0.

use crate::r_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::blockchain::BlockchainConfig;
use crate::blockchain::ElectrumConfig;
use crate::blockchain::EsploraConfig;
use crate::blockchain::RpcConfig;
use crate::blockchain::RpcSyncParams;
use crate::blockchain::UserPass;
use crate::types::AddressIndex;
use crate::types::AddressInfo;
use crate::types::Balance;
use crate::types::BdkTxBuilderResult;
use crate::types::BlockTime;
use crate::types::KeychainKind;
use crate::types::Network;
use crate::types::OutPoint;
use crate::types::PackedLockTime;
use crate::types::ScriptAmount;
use crate::types::Sequence;
use crate::types::Transaction2;
use crate::types::TransactionDetails;
use crate::types::TxIn;
use crate::types::TxOut;
use crate::types::WordCount;
use crate::wallet::DatabaseConfig;
use crate::wallet::LocalUtxo;
use crate::wallet::SledDbConfiguration;
use crate::wallet::SqliteDbConfiguration;

// Section: wire functions

fn wire_blockchain_init_impl(
    port_: MessagePort,
    config: impl Wire2Api<BlockchainConfig> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "blockchain_init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_config = config.wire2api();
            move |task_callback| blockchain_init(api_config)
        },
    )
}
fn wire_get_blockchain_height_impl(
    port_: MessagePort,
    blockchain: impl Wire2Api<RustOpaque<BlockchainInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_blockchain_height",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_blockchain = blockchain.wire2api();
            move |task_callback| get_blockchain_height(api_blockchain)
        },
    )
}
fn wire_get_blockchain_hash_impl(
    port_: MessagePort,
    blockchain_height: impl Wire2Api<u32> + UnwindSafe,
    blockchain: impl Wire2Api<RustOpaque<BlockchainInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_blockchain_hash",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_blockchain_height = blockchain_height.wire2api();
            let api_blockchain = blockchain.wire2api();
            move |task_callback| get_blockchain_hash(api_blockchain_height, api_blockchain)
        },
    )
}
fn wire_estimate_fee_impl(
    port_: MessagePort,
    target: impl Wire2Api<u64> + UnwindSafe,
    blockchain: impl Wire2Api<RustOpaque<BlockchainInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "estimate_fee",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_target = target.wire2api();
            let api_blockchain = blockchain.wire2api();
            move |task_callback| estimate_fee(api_target, api_blockchain)
        },
    )
}
fn wire_broadcast_impl(
    port_: MessagePort,
    tx: impl Wire2Api<Vec<u8>> + UnwindSafe,
    blockchain: impl Wire2Api<RustOpaque<BlockchainInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "broadcast",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_tx = tx.wire2api();
            let api_blockchain = blockchain.wire2api();
            move |task_callback| broadcast(api_tx, api_blockchain)
        },
    )
}
fn wire_new_transaction_impl(port_: MessagePort, tx: impl Wire2Api<Vec<u8>> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_transaction",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_tx = tx.wire2api();
            move |task_callback| new_transaction(api_tx)
        },
    )
}
fn wire_psbt_to_txid_impl(port_: MessagePort, psbt_str: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "psbt_to_txid",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_psbt_str = psbt_str.wire2api();
            move |task_callback| psbt_to_txid(api_psbt_str)
        },
    )
}
fn wire_extract_tx_impl(port_: MessagePort, psbt_str: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "extract_tx",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_psbt_str = psbt_str.wire2api();
            move |task_callback| extract_tx(api_psbt_str)
        },
    )
}
fn wire_get_psbt_fee_rate_impl(port_: MessagePort, psbt_str: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_psbt_fee_rate",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_psbt_str = psbt_str.wire2api();
            move |task_callback| Ok(get_psbt_fee_rate(api_psbt_str))
        },
    )
}
fn wire_get_fee_amount_impl(port_: MessagePort, psbt_str: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_fee_amount",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_psbt_str = psbt_str.wire2api();
            move |task_callback| Ok(get_fee_amount(api_psbt_str))
        },
    )
}
fn wire_combine_psbt_impl(
    port_: MessagePort,
    psbt_str: impl Wire2Api<String> + UnwindSafe,
    other: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "combine_psbt",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_psbt_str = psbt_str.wire2api();
            let api_other = other.wire2api();
            move |task_callback| combine_psbt(api_psbt_str, api_other)
        },
    )
}
fn wire_tx_builder_finish_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
    recipients: impl Wire2Api<Vec<ScriptAmount>> + UnwindSafe,
    utxos: impl Wire2Api<Vec<OutPoint>> + UnwindSafe,
    unspendable: impl Wire2Api<Vec<OutPoint>> + UnwindSafe,
    manually_selected_only: impl Wire2Api<bool> + UnwindSafe,
    only_spend_change: impl Wire2Api<bool> + UnwindSafe,
    do_not_spend_change: impl Wire2Api<bool> + UnwindSafe,
    fee_rate: impl Wire2Api<Option<f32>> + UnwindSafe,
    fee_absolute: impl Wire2Api<Option<u64>> + UnwindSafe,
    drain_wallet: impl Wire2Api<bool> + UnwindSafe,
    drain_to: impl Wire2Api<Option<String>> + UnwindSafe,
    enable_rbf: impl Wire2Api<bool> + UnwindSafe,
    n_sequence: impl Wire2Api<Option<u32>> + UnwindSafe,
    data: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "tx_builder_finish",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            let api_recipients = recipients.wire2api();
            let api_utxos = utxos.wire2api();
            let api_unspendable = unspendable.wire2api();
            let api_manually_selected_only = manually_selected_only.wire2api();
            let api_only_spend_change = only_spend_change.wire2api();
            let api_do_not_spend_change = do_not_spend_change.wire2api();
            let api_fee_rate = fee_rate.wire2api();
            let api_fee_absolute = fee_absolute.wire2api();
            let api_drain_wallet = drain_wallet.wire2api();
            let api_drain_to = drain_to.wire2api();
            let api_enable_rbf = enable_rbf.wire2api();
            let api_n_sequence = n_sequence.wire2api();
            let api_data = data.wire2api();
            move |task_callback| {
                tx_builder_finish(
                    api_wallet,
                    api_recipients,
                    api_utxos,
                    api_unspendable,
                    api_manually_selected_only,
                    api_only_spend_change,
                    api_do_not_spend_change,
                    api_fee_rate,
                    api_fee_absolute,
                    api_drain_wallet,
                    api_drain_to,
                    api_enable_rbf,
                    api_n_sequence,
                    api_data,
                )
            }
        },
    )
}
fn wire_bump_fee_tx_builder_finish_impl(
    port_: MessagePort,
    txid: impl Wire2Api<String> + UnwindSafe,
    fee_rate: impl Wire2Api<f32> + UnwindSafe,
    allow_shrinking: impl Wire2Api<Option<String>> + UnwindSafe,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
    enable_rbf: impl Wire2Api<bool> + UnwindSafe,
    n_sequence: impl Wire2Api<Option<u32>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "bump_fee_tx_builder_finish",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_txid = txid.wire2api();
            let api_fee_rate = fee_rate.wire2api();
            let api_allow_shrinking = allow_shrinking.wire2api();
            let api_wallet = wallet.wire2api();
            let api_enable_rbf = enable_rbf.wire2api();
            let api_n_sequence = n_sequence.wire2api();
            move |task_callback| {
                bump_fee_tx_builder_finish(
                    api_txid,
                    api_fee_rate,
                    api_allow_shrinking,
                    api_wallet,
                    api_enable_rbf,
                    api_n_sequence,
                )
            }
        },
    )
}
fn wire_new_descriptor_impl(
    port_: MessagePort,
    descriptor: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_descriptor",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_descriptor = descriptor.wire2api();
            let api_network = network.wire2api();
            move |task_callback| new_descriptor(api_descriptor, api_network)
        },
    )
}
fn wire_new_bip44_descriptor_impl(
    port_: MessagePort,
    key_chain_kind: impl Wire2Api<KeychainKind> + UnwindSafe,
    secret_key: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_bip44_descriptor",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_key_chain_kind = key_chain_kind.wire2api();
            let api_secret_key = secret_key.wire2api();
            let api_network = network.wire2api();
            move |task_callback| {
                Ok(new_bip44_descriptor(
                    api_key_chain_kind,
                    api_secret_key,
                    api_network,
                ))
            }
        },
    )
}
fn wire_new_bip44_public_impl(
    port_: MessagePort,
    key_chain_kind: impl Wire2Api<KeychainKind> + UnwindSafe,
    public_key: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
    fingerprint: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_bip44_public",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_key_chain_kind = key_chain_kind.wire2api();
            let api_public_key = public_key.wire2api();
            let api_network = network.wire2api();
            let api_fingerprint = fingerprint.wire2api();
            move |task_callback| {
                Ok(new_bip44_public(
                    api_key_chain_kind,
                    api_public_key,
                    api_network,
                    api_fingerprint,
                ))
            }
        },
    )
}
fn wire_new_bip49_descriptor_impl(
    port_: MessagePort,
    key_chain_kind: impl Wire2Api<KeychainKind> + UnwindSafe,
    secret_key: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_bip49_descriptor",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_key_chain_kind = key_chain_kind.wire2api();
            let api_secret_key = secret_key.wire2api();
            let api_network = network.wire2api();
            move |task_callback| {
                Ok(new_bip49_descriptor(
                    api_key_chain_kind,
                    api_secret_key,
                    api_network,
                ))
            }
        },
    )
}
fn wire_new_bip49_public_impl(
    port_: MessagePort,
    key_chain_kind: impl Wire2Api<KeychainKind> + UnwindSafe,
    public_key: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
    fingerprint: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_bip49_public",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_key_chain_kind = key_chain_kind.wire2api();
            let api_public_key = public_key.wire2api();
            let api_network = network.wire2api();
            let api_fingerprint = fingerprint.wire2api();
            move |task_callback| {
                Ok(new_bip49_public(
                    api_key_chain_kind,
                    api_public_key,
                    api_network,
                    api_fingerprint,
                ))
            }
        },
    )
}
fn wire_new_bip84_descriptor_impl(
    port_: MessagePort,
    key_chain_kind: impl Wire2Api<KeychainKind> + UnwindSafe,
    secret_key: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_bip84_descriptor",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_key_chain_kind = key_chain_kind.wire2api();
            let api_secret_key = secret_key.wire2api();
            let api_network = network.wire2api();
            move |task_callback| {
                Ok(new_bip84_descriptor(
                    api_key_chain_kind,
                    api_secret_key,
                    api_network,
                ))
            }
        },
    )
}
fn wire_new_bip84_public_impl(
    port_: MessagePort,
    key_chain_kind: impl Wire2Api<KeychainKind> + UnwindSafe,
    public_key: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
    fingerprint: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_bip84_public",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_key_chain_kind = key_chain_kind.wire2api();
            let api_public_key = public_key.wire2api();
            let api_network = network.wire2api();
            let api_fingerprint = fingerprint.wire2api();
            move |task_callback| {
                Ok(new_bip84_public(
                    api_key_chain_kind,
                    api_public_key,
                    api_network,
                    api_fingerprint,
                ))
            }
        },
    )
}
fn wire_as_string_private_impl(
    port_: MessagePort,
    descriptor: impl Wire2Api<RustOpaque<BdkDescriptor>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_string_private",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_descriptor = descriptor.wire2api();
            move |task_callback| Ok(as_string_private(api_descriptor))
        },
    )
}
fn wire_as_string_impl(
    port_: MessagePort,
    descriptor: impl Wire2Api<RustOpaque<BdkDescriptor>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_string",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_descriptor = descriptor.wire2api();
            move |task_callback| Ok(as_string(api_descriptor))
        },
    )
}
fn wire_create_descriptor_secret_impl(
    port_: MessagePort,
    network: impl Wire2Api<Network> + UnwindSafe,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
    password: impl Wire2Api<Option<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create_descriptor_secret",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_network = network.wire2api();
            let api_mnemonic = mnemonic.wire2api();
            let api_password = password.wire2api();
            move |task_callback| create_descriptor_secret(api_network, api_mnemonic, api_password)
        },
    )
}
fn wire_descriptor_secret_from_string_impl(
    port_: MessagePort,
    xprv: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "descriptor_secret_from_string",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_xprv = xprv.wire2api();
            move |task_callback| descriptor_secret_from_string(api_xprv)
        },
    )
}
fn wire_descriptor_secret_extend_impl(
    port_: MessagePort,
    xprv: impl Wire2Api<String> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "descriptor_secret_extend",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_xprv = xprv.wire2api();
            let api_path = path.wire2api();
            move |task_callback| Ok(descriptor_secret_extend(api_xprv, api_path))
        },
    )
}
fn wire_descriptor_secret_derive_impl(
    port_: MessagePort,
    xprv: impl Wire2Api<String> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "descriptor_secret_derive",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_xprv = xprv.wire2api();
            let api_path = path.wire2api();
            move |task_callback| Ok(descriptor_secret_derive(api_xprv, api_path))
        },
    )
}
fn wire_descriptor_secret_as_secret_bytes_impl(
    port_: MessagePort,
    descriptor_secret: impl Wire2Api<Option<String>> + UnwindSafe,
    xprv: impl Wire2Api<Option<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "descriptor_secret_as_secret_bytes",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_descriptor_secret = descriptor_secret.wire2api();
            let api_xprv = xprv.wire2api();
            move |task_callback| descriptor_secret_as_secret_bytes(api_descriptor_secret, api_xprv)
        },
    )
}
fn wire_descriptor_secret_as_public_impl(
    port_: MessagePort,
    descriptor_secret: impl Wire2Api<Option<String>> + UnwindSafe,
    xprv: impl Wire2Api<Option<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "descriptor_secret_as_public",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_descriptor_secret = descriptor_secret.wire2api();
            let api_xprv = xprv.wire2api();
            move |task_callback| descriptor_secret_as_public(api_descriptor_secret, api_xprv)
        },
    )
}
fn wire_create_derivation_path_impl(port_: MessagePort, path: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create_derivation_path",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| create_derivation_path(api_path)
        },
    )
}
fn wire_descriptor_public_from_string_impl(
    port_: MessagePort,
    public_key: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "descriptor_public_from_string",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_public_key = public_key.wire2api();
            move |task_callback| descriptor_public_from_string(api_public_key)
        },
    )
}
fn wire_create_descriptor_public_impl(
    port_: MessagePort,
    xpub: impl Wire2Api<Option<String>> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
    derive: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create_descriptor_public",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_xpub = xpub.wire2api();
            let api_path = path.wire2api();
            let api_derive = derive.wire2api();
            move |task_callback| create_descriptor_public(api_xpub, api_path, api_derive)
        },
    )
}
fn wire_init_script_impl(
    port_: MessagePort,
    raw_output_script: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init_script",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_raw_output_script = raw_output_script.wire2api();
            move |task_callback| init_script(api_raw_output_script)
        },
    )
}
fn wire_init_address_impl(port_: MessagePort, address: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init_address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_address = address.wire2api();
            move |task_callback| init_address(api_address)
        },
    )
}
fn wire_address_to_script_pubkey_hex_impl(
    port_: MessagePort,
    address: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "address_to_script_pubkey_hex",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_address = address.wire2api();
            move |task_callback| Ok(address_to_script_pubkey_hex(api_address))
        },
    )
}
fn wire_wallet_init_impl(
    port_: MessagePort,
    descriptor: impl Wire2Api<RustOpaque<BdkDescriptor>> + UnwindSafe,
    change_descriptor: impl Wire2Api<Option<RustOpaque<BdkDescriptor>>> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
    database_config: impl Wire2Api<DatabaseConfig> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "wallet_init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_descriptor = descriptor.wire2api();
            let api_change_descriptor = change_descriptor.wire2api();
            let api_network = network.wire2api();
            let api_database_config = database_config.wire2api();
            move |task_callback| {
                wallet_init(
                    api_descriptor,
                    api_change_descriptor,
                    api_network,
                    api_database_config,
                )
            }
        },
    )
}
fn wire_get_address_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
    address_index: impl Wire2Api<AddressIndex> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            let api_address_index = address_index.wire2api();
            move |task_callback| get_address(api_wallet, api_address_index)
        },
    )
}
fn wire_get_internalized_address_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
    address_index: impl Wire2Api<AddressIndex> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_internalized_address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            let api_address_index = address_index.wire2api();
            move |task_callback| get_internalized_address(api_wallet, api_address_index)
        },
    )
}
fn wire_sync_wallet_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
    blockchain: impl Wire2Api<RustOpaque<BlockchainInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sync_wallet",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            let api_blockchain = blockchain.wire2api();
            move |task_callback| Ok(sync_wallet(api_wallet, api_blockchain))
        },
    )
}
fn wire_get_balance_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_balance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            move |task_callback| get_balance(api_wallet)
        },
    )
}
fn wire_list_unspent_outputs_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "list_unspent_outputs",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            move |task_callback| list_unspent_outputs(api_wallet)
        },
    )
}
fn wire_get_transactions_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_transactions",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            move |task_callback| get_transactions(api_wallet)
        },
    )
}
fn wire_sign_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
    psbt_str: impl Wire2Api<String> + UnwindSafe,
    is_multi_sig: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sign",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            let api_psbt_str = psbt_str.wire2api();
            let api_is_multi_sig = is_multi_sig.wire2api();
            move |task_callback| Ok(sign(api_wallet, api_psbt_str, api_is_multi_sig))
        },
    )
}
fn wire_get_network_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_network",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            move |task_callback| Ok(get_network(api_wallet))
        },
    )
}
fn wire_list_unspent_impl(
    port_: MessagePort,
    wallet: impl Wire2Api<RustOpaque<WalletInstance>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "list_unspent",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet = wallet.wire2api();
            move |task_callback| list_unspent(api_wallet)
        },
    )
}
fn wire_generate_seed_from_word_count_impl(
    port_: MessagePort,
    word_count: impl Wire2Api<WordCount> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "generate_seed_from_word_count",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_word_count = word_count.wire2api();
            move |task_callback| Ok(generate_seed_from_word_count(api_word_count))
        },
    )
}
fn wire_generate_seed_from_string_impl(
    port_: MessagePort,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "generate_seed_from_string",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mnemonic = mnemonic.wire2api();
            move |task_callback| generate_seed_from_string(api_mnemonic)
        },
    )
}
fn wire_generate_seed_from_entropy_impl(
    port_: MessagePort,
    entropy: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "generate_seed_from_entropy",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_entropy = entropy.wire2api();
            move |task_callback| generate_seed_from_entropy(api_entropy)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<KeychainKind> for i32 {
    fn wire2api(self) -> KeychainKind {
        match self {
            0 => KeychainKind::External,
            1 => KeychainKind::Internal,
            _ => unreachable!("Invalid variant for KeychainKind: {}", self),
        }
    }
}

impl Wire2Api<Network> for i32 {
    fn wire2api(self) -> Network {
        match self {
            0 => Network::Testnet,
            1 => Network::Regtest,
            2 => Network::Bitcoin,
            3 => Network::Signet,
            _ => unreachable!("Invalid variant for Network: {}", self),
        }
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<WordCount> for i32 {
    fn wire2api(self) -> WordCount {
        match self {
            0 => WordCount::Words12,
            1 => WordCount::Words18,
            2 => WordCount::Words24,
            _ => unreachable!("Invalid variant for WordCount: {}", self),
        }
    }
}
// Section: impl IntoDart

impl support::IntoDart for AddressInfo {
    fn into_dart(self) -> support::DartAbi {
        vec![self.index.into_dart(), self.address.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for AddressInfo {}

impl support::IntoDart for Balance {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.immature.into_dart(),
            self.trusted_pending.into_dart(),
            self.untrusted_pending.into_dart(),
            self.confirmed.into_dart(),
            self.spendable.into_dart(),
            self.total.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Balance {}

impl support::IntoDart for BdkTxBuilderResult {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart(), self.1.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BdkTxBuilderResult {}

impl support::IntoDart for BlockTime {
    fn into_dart(self) -> support::DartAbi {
        vec![self.height.into_dart(), self.timestamp.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BlockTime {}

impl support::IntoDart for LocalUtxo {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.outpoint.into_dart(),
            self.txout.into_dart(),
            self.is_spent.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LocalUtxo {}

impl support::IntoDart for Network {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Testnet => 0,
            Self::Regtest => 1,
            Self::Bitcoin => 2,
            Self::Signet => 3,
        }
        .into_dart()
    }
}

impl support::IntoDart for OutPoint {
    fn into_dart(self) -> support::DartAbi {
        vec![self.txid.into_dart(), self.vout.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OutPoint {}

impl support::IntoDart for PackedLockTime {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PackedLockTime {}

impl support::IntoDart for Sequence {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Sequence {}

impl support::IntoDart for Transaction2 {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.version.into_dart(),
            self.lock_time.into_dart(),
            self.input.into_dart(),
            self.output.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Transaction2 {}

impl support::IntoDart for TransactionDetails {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.transaction.into_dart(),
            self.txid.into_dart(),
            self.received.into_dart(),
            self.sent.into_dart(),
            self.fee.into_dart(),
            self.confirmation_time.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TransactionDetails {}

impl support::IntoDart for TxIn {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.previous_output.into_dart(),
            self.script_sig.into_dart(),
            self.sequence.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TxIn {}

impl support::IntoDart for TxOut {
    fn into_dart(self) -> support::DartAbi {
        vec![self.value.into_dart(), self.address.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TxOut {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
