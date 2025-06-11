use crate::taproot::{
    TaprootBuilder,
    TaprootMerkleBranch,
    LeafVersion,
    TapLeafHash,
    TapNodeHash,
    NodeInfo,
    LeafNode,
    ControlBlock
};
use crate::blockdata::opcodes::all::*;
use crate::blockdata::script::{
    ScriptBuf,
    Builder
};
use crate::hashes::Hash;

pub struct P2qrhScriptBuf {
    inner: ScriptBuf
}

impl P2qrhScriptBuf {
    pub fn new(inner: ScriptBuf) -> Self {
        Self { inner }
    }
    
    /// Generates P2QRH scriptPubKey output
    /// Only accepts the merkle_root (of type TapNodeHash) since keypath spend is disabled in p2qrh
    pub fn new_p2qrh(merkle_root: TapNodeHash) -> Self {
        // https://github.com/cryptoquick/bips/blob/p2qrh/bip-0360.mediawiki#scriptpubkey
        let merkle_root_hash_bytes: [u8; 32] = merkle_root.to_byte_array();
        let script = Builder::new()
            .push_opcode(OP_PUSHNUM_3)
            .push_opcode(OP_PUSHBYTES_32)
            .push_slice(&merkle_root_hash_bytes)
            .into_script();
        P2qrhScriptBuf::new(script)
    }
}

pub struct P2qrhBuilder {
    inner: TaprootBuilder
}

impl P2qrhBuilder {
    pub fn new() -> Self {
        Self {
            inner: TaprootBuilder::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct P2pqrhSpendInfo {
    // We'll use the same types from taproot module
    merkle_root: Option<TapNodeHash>,
    //script_map: BTreeMap<(ScriptBuf, LeafVersion), BTreeSet<TaprootMerkleBranch>>,
}

impl P2pqrhSpendInfo {
    /*
    pub fn control_block(&self, script_ver: &(ScriptBuf, LeafVersion)) -> Option<P2pqrhControlBlock> {
        // Create our own control block type that doesn't include key information
        if let Some(merkle_branch) = self.script_map.get(script_ver) {
            Some(P2pqrhControlBlock {
                leaf_version: script_ver.1,
                merkle_branch: merkle_branch.iter().next().unwrap().clone(),
            })
        } else {
            None
        }
    }*/
}

// Our simplified control block without key-related fields
pub struct P2pqrhControlBlock {
    pub leaf_version: LeafVersion,
    pub merkle_branch: TaprootMerkleBranch,
}