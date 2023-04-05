use bdk::bitcoin::blockdata::script::Script as BdkScript;
use bdk::bitcoin::hashes::hex::{FromHex, ToHex};
use bdk::bitcoin::{Address as BdkAddress, OutPoint as BdkOutPoint, Txid};
use bdk::{Balance as BdkBalance, Error as BdkError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

/// Bitcoin transaction input.
///
/// It contains the location of the previous transaction's output,
/// that it spends and set of scripts that satisfy its spending
/// conditions.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TxIn {
    /// The reference to the previous output that is being used an an input.
    pub previous_output: OutPoint,
    /// The script which pushes values on the stack which will cause
    /// the referenced output's script to be accepted.
    pub script_sig: String,
    /// The sequence number, which suggests to miners which of two
    /// conflicting transactions should be preferred, or 0xFFFFFFFF
    /// to ignore this feature. This is generally never used since
    /// the miner behaviour cannot be enforced.
    pub sequence: Sequence,
    // pub witness: Vec<u8>,
}
impl From<bdk::bitcoin::TxIn> for TxIn {
    fn from(x: bdk::bitcoin::TxIn) -> Self {
        TxIn {
            previous_output: x.previous_output.into(),
            script_sig: x.script_sig.to_string(),
            sequence: x.sequence.into(),
            // witness: x.witness.to_vec(),
        }
    }
}
///A transaction output, which defines new coins to be created from old ones.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: u64,
    /// The address of the output.
    pub address: String,
}
impl From<bdk::bitcoin::TxOut> for TxOut {
    fn from(x: bdk::bitcoin::TxOut) -> Self {
        TxOut {
            value: x.value,
            address: x.script_pubkey.to_string(),
        }
    }
}
/// A reference to a transaction output.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OutPoint {
    /// The referenced transaction's txid.
    pub(crate) txid: String,
    /// The index of the referenced output in its transaction's vout.
    pub(crate) vout: u32,
}
impl From<&OutPoint> for BdkOutPoint {
    fn from(x: &OutPoint) -> BdkOutPoint {
        BdkOutPoint {
            txid: Txid::from_str(&x.clone().txid).unwrap(),
            vout: x.clone().vout,
        }
    }
}
impl From<bdk::bitcoin::OutPoint> for OutPoint {
    fn from(x: bdk::bitcoin::OutPoint) -> Self {
        OutPoint {
            txid: x.txid.to_string(),
            vout: x.clone().vout,
        }
    }
}

/// Local Wallet's Balance
#[derive(Deserialize)]
pub struct Balance {
    // All coinbase outputs not yet matured
    pub immature: u64,
    /// Unconfirmed UTXOs generated by a wallet tx
    pub trusted_pending: u64,
    /// Unconfirmed UTXOs received from an external wallet
    pub untrusted_pending: u64,
    /// Confirmed and immediately spendable balance
    pub confirmed: u64,
    /// Get sum of trusted_pending and confirmed coins
    pub spendable: u64,
    /// Get the whole balance visible to the wallet
    pub total: u64,
}
impl From<BdkBalance> for Balance {
    fn from(bdk_balance: BdkBalance) -> Self {
        Balance {
            immature: bdk_balance.immature,
            trusted_pending: bdk_balance.trusted_pending,
            untrusted_pending: bdk_balance.untrusted_pending,
            confirmed: bdk_balance.confirmed,
            spendable: bdk_balance.get_spendable(),
            total: bdk_balance.get_total(),
        }
    }
}

/// The address index selection strategy to use to derived an address from the wallet's external
/// descriptor.
pub enum AddressIndex {
    ///Return a new address after incrementing the current descriptor index.
    New,
    ///Return the address for the current descriptor index if it has not been used in a received transaction. Otherwise return a new address as with AddressIndex.New.
    ///Use with caution, if the wallet has not yet detected an address has been used it could return an already used address. This function is primarily meant for situations where the caller is untrusted; for example when deriving donation addresses on-demand for a public web page.
    LastUnused,
    /// Return the address for a specific descriptor index. Does not change the current descriptor
    /// index used by `AddressIndex` and `AddressIndex.LastUsed`.
    /// Use with caution, if an index is given that is less than the current descriptor index
    /// then the returned address may have already been used.
    Peek { index: u32 },
    /// Return the address for a specific descriptor index and reset the current descriptor index
    /// used by `AddressIndex` and `AddressIndex.LastUsed` to this value.
    /// Use with caution, if an index is given that is less than the current descriptor index
    /// then the returned address and subsequent addresses returned by calls to `AddressIndex`
    /// and `AddressIndex.LastUsed` may have already been used. Also if the index is reset to a
    /// value earlier than the Blockchain stopGap (default is 20) then a
    /// larger stopGap should be used to monitor for all possibly used addresses.
    Reset { index: u32 },
}
impl From<AddressIndex> for bdk::wallet::AddressIndex {
    fn from(x: AddressIndex) -> bdk::wallet::AddressIndex {
        match x {
            AddressIndex::New => bdk::wallet::AddressIndex::New,
            AddressIndex::LastUnused => bdk::wallet::AddressIndex::LastUnused,
            AddressIndex::Peek { index } => bdk::wallet::AddressIndex::Peek(index),
            AddressIndex::Reset { index } => bdk::wallet::AddressIndex::Reset(index),
        }
    }
}

///A derived address and the index it was found at For convenience this automatically derefs to Address
pub struct AddressInfo {
    ///Child index of this address
    pub index: u32,
    /// Address
    pub address: String,
}
impl From<bdk::wallet::AddressInfo> for AddressInfo {
    fn from(x: bdk::wallet::AddressInfo) -> AddressInfo {
        AddressInfo {
            index: x.index,
            address: x.address.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Transaction2 {
    /// The protocol version, is currently expected to be 1 or 2 (BIP 68).
    pub version: i32,
    /// Block height or timestamp. Transaction cannot be included in a block until this height/time.
    /// ### Relevant BIPs
    /// * [BIP-65 OP_CHECKLOCKTIMEVERIFY](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)
    /// * [BIP-113 Median time-past as endpoint for lock-time calculations](https://github.com/bitcoin/bips/blob/master/bip-0113.mediawiki)
    pub lock_time: PackedLockTime,
    /// List of transaction inputs.
    pub input: Vec<TxIn>,
    /// List of transaction outputs.
    pub output: Vec<TxOut>,
}

impl From<bdk::bitcoin::Transaction> for Transaction2 {
    fn from(x: bdk::bitcoin::Transaction) -> Self {
        Transaction2 {
            version: x.version,
            lock_time: x.lock_time.into(),
            input: set_inputs(x.input),
            output: set_outputs(x.output),
        }
    }
}
fn set_inputs(x: Vec<bdk::bitcoin::TxIn>) -> Vec<TxIn> {
    let mut vec = Vec::new();
    for input in x {
        vec.push(input.into())
    }
    return vec;
}
fn set_outputs(x: Vec<bdk::bitcoin::TxOut>) -> Vec<TxOut> {
    let mut vec = Vec::new();
    for output in x {
        vec.push(output.into())
    }
    return vec;
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
///A wallet transaction
pub struct TransactionDetails {
    pub transaction: Option<Transaction2>,
    /// Transaction id.
    pub txid: String,
    /// Received value (sats)
    /// Sum of owned outputs of this transaction.
    pub received: u64,
    /// Sent value (sats)
    /// Sum of owned inputs of this transaction.
    pub sent: u64,
    /// Fee value (sats) if confirmed.
    /// The availability of the fee depends on the backend. It's never None with an Electrum
    /// Server backend, but it could be None with a Bitcoin RPC node without txindex that receive
    /// funds while offline.
    pub fee: Option<u64>,
    /// If the transaction is confirmed, contains height and timestamp of the block containing the
    /// transaction, unconfirmed transaction contains `None`.
    pub confirmation_time: Option<BlockTime>,
}
impl From<&bdk::TransactionDetails> for TransactionDetails {
    fn from(x: &bdk::TransactionDetails) -> TransactionDetails {
        TransactionDetails {
            transaction: set_transaction(x.transaction.clone()),
            fee: x.clone().fee,
            txid: x.clone().txid.to_string(),
            received: x.clone().received,
            sent: x.clone().sent,
            confirmation_time: set_block_time(x.confirmation_time.clone()),
        }
    }
}
fn set_transaction(transaction: Option<bdk::bitcoin::Transaction>) -> Option<Transaction2> {
    if let Some(transaction) = transaction {
        Some(transaction.into())
    } else {
        None
    }
}
fn set_block_time(time: Option<bdk::BlockTime>) -> Option<BlockTime> {
    if let Some(time) = time {
        Some(time.into())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
///Block height and timestamp of a block
pub struct BlockTime {
    ///Confirmation block height
    pub height: u32,
    ///Confirmation block timestamp
    pub timestamp: u64,
}
impl From<bdk::BlockTime> for BlockTime {
    fn from(x: bdk::BlockTime) -> Self {
        BlockTime {
            height: x.height,
            timestamp: x.timestamp,
        }
    }
}

/// A output script and an amount of satoshis.
pub struct ScriptAmount {
    pub script: String,
    pub amount: u64,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum RbfValue {
    RbfDefault,
    Value(u32),
}

/// The result after calling the TxBuilder finish() function. Contains unsigned PSBT and
/// transaction details.
pub struct BdkTxBuilderResult(pub String, pub TransactionDetails);

///Types of keychains
pub enum KeychainKind {
    External,
    ///Internal, usually used for change outputs
    Internal,
}
impl From<bdk::KeychainKind> for KeychainKind {
    fn from(e: bdk::KeychainKind) -> Self {
        match e {
            bdk::KeychainKind::External => KeychainKind::External,
            bdk::KeychainKind::Internal => KeychainKind::Internal,
        }
    }
}
impl From<KeychainKind> for bdk::KeychainKind {
    fn from(kind: KeychainKind) -> Self {
        match kind {
            KeychainKind::External => bdk::KeychainKind::External,
            KeychainKind::Internal => bdk::KeychainKind::Internal,
        }
    }
}

#[derive(Clone)]
///The cryptocurrency to act on
pub enum Network {
    ///Bitcoin’s testnet
    Testnet,
    ///Bitcoin’s regtest
    Regtest,
    ///Classic Bitcoin
    Bitcoin,
    ///Bitcoin’s signet
    Signet,
}
impl Default for Network {
    fn default() -> Self {
        Network::Testnet
    }
}
impl From<Network> for bdk::bitcoin::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Signet => bdk::bitcoin::Network::Signet,
            Network::Testnet => bdk::bitcoin::Network::Testnet,
            Network::Regtest => bdk::bitcoin::Network::Regtest,
            Network::Bitcoin => bdk::bitcoin::Network::Bitcoin,
        }
    }
}
impl From<bdk::bitcoin::Network> for Network {
    fn from(network: bdk::bitcoin::Network) -> Self {
        match network {
            bdk::bitcoin::Network::Signet => Network::Signet,
            bdk::bitcoin::Network::Testnet => Network::Testnet,
            bdk::bitcoin::Network::Regtest => Network::Regtest,
            bdk::bitcoin::Network::Bitcoin => Network::Bitcoin,
        }
    }
}

///Type describing entropy length (aka word count) in the mnemonic
pub enum WordCount {
    ///12 words mnemonic (128 bits entropy)
    Words12,
    ///18 words mnemonic (192 bits entropy)
    Words18,
    ///24 words mnemonic (256 bits entropy)
    Words24,
}
impl From<WordCount> for bdk::keys::bip39::WordCount {
    fn from(word_count: WordCount) -> Self {
        match word_count {
            WordCount::Words12 => bdk::keys::bip39::WordCount::Words12,
            WordCount::Words18 => bdk::keys::bip39::WordCount::Words18,
            WordCount::Words24 => bdk::keys::bip39::WordCount::Words24,
        }
    }
}
pub struct Address {
    pub address: BdkAddress,
}
impl Address {
    pub fn new(address: String) -> Result<Self, BdkError> {
        BdkAddress::from_str(address.as_str())
            .map(|a| Address { address: a })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    pub fn script_pubkey(&self) -> Arc<Script> {
        Arc::new(Script {
            script: self.address.script_pubkey(),
        })
    }
}
/// A Bitcoin script.
#[derive(Clone)]
pub struct Script {
    pub script: BdkScript,
}
impl Script {
    //Custom function for rApi
    pub fn from_hex(script: String) -> Result<Self, BdkError> {
        let script = BdkScript::from_hex(script.as_str()).unwrap();
        Ok(Script { script })
    }
    pub fn new(raw_output_script: Vec<u8>) -> Result<Self, BdkError> {
        let script = raw_output_script.as_slice().to_hex();
        Script::from_hex(script)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Sequence(pub u32);
impl From<bdk::bitcoin::Sequence> for Sequence {
    fn from(x: bdk::bitcoin::Sequence) -> Self {
        Sequence(x.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PackedLockTime(pub u32);
impl From<bdk::bitcoin::PackedLockTime> for PackedLockTime {
    fn from(x: bdk::bitcoin::PackedLockTime) -> Self {
        PackedLockTime(x.0)
    }
}
