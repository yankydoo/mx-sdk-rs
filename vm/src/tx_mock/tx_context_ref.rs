use std::{ops::Deref, rc::Rc};

use crate::tx_mock::{ApiCalls, TxContext, TxResult};

use super::{BlockchainUpdate, TxContextStack};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Debug)]
pub struct TxContextRef(Rc<TxContext>);

pub type DebugApi = TxContextRef;

impl Deref for TxContextRef {
    type Target = TxContext;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Clone for TxContextRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl TxContextRef {
    pub fn new(tx_context_rc: Rc<TxContext>) -> Self {
        Self(tx_context_rc)
    }

    pub fn new_from_static() -> Self {
        let tx_context_rc = TxContextStack::static_peek();
        Self(tx_context_rc)
    }

    pub fn dummy() -> Self {
        let tx_context = TxContext::dummy();
        let tx_context_rc = Rc::new(tx_context);
        // TODO: WARNING: this does not clean up after itself, must fix!!!
        TxContextStack::static_push(tx_context_rc.clone());
        Self(tx_context_rc)
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        let tx_context = Rc::try_unwrap(self.0).unwrap();
        let tx_cache = Rc::try_unwrap(tx_context.tx_cache).unwrap();
        tx_cache.into_blockchain_updates()
    }

    /// Consumes the current API and returns the contained output.
    /// Should be called at the end of a tx execution.
    /// Will fail if any other references to the tx context survive, this must be the last.
    pub fn into_tx_result(self) -> TxResult {
        // TODO: investigate if we can also destroy the Rc
        // can be done if we can make sure that no more references exist at this point
        // let tx_context = Rc::try_unwrap(self.0).unwrap();
        self.tx_result_cell.replace(TxResult::default())
    }

    /// Will yield a copy of all messages printed on this context.
    pub fn printed_messages(&self) -> Vec<String> {
        self.0.printed_messages.borrow().clone()
    }

    /// Clears entire print history.
    pub fn printed_messages_clear(&self) {
        self.0.printed_messages.borrow_mut().clear();
    }

    pub fn count_transfers(num: usize) {
        let tx_context_rc = TxContextStack::static_peek();
        tx_context_rc.api_calls.borrow_mut().transfers += num;
    }
    pub fn count_trie_read() {
        let tx_context_rc = TxContextStack::static_peek();
        tx_context_rc.api_calls.borrow_mut().trie_reads += 1;
    }
    pub fn count_built_in_call() {
        let tx_context_rc = TxContextStack::static_peek();
        tx_context_rc.api_calls.borrow_mut().built_in_calls += 1;
    }

    pub fn get_api_calls(&self) -> ApiCalls {
        let tx_context_rc = TxContextStack::static_peek();
        let x = (*tx_context_rc.api_calls.borrow()).clone(); x
    }
}
