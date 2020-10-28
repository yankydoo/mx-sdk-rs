use crate::*;
use elrond_codec::TopDecodeInput;
use core::marker::PhantomData;

/// Adapter from the API to the TopDecodeInput trait.
/// Allows objects to be deserialized directly from the API as arguments.
/// 
/// Of course the implementation provides shortcut deserialization computation paths directly from API:
/// into_u64, into_i64, ...
/// 
/// This is a performance-critical struct.
/// Since the wasm ContractIOApi is zero-size,
/// it means that this structures translates to a single glorified i32 in wasm.
pub struct ArgDecodeInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    api: &'a A,
    arg_index: i32,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> ArgDecodeInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    #[inline]
    pub fn new(api: &'a A, arg_index: i32) -> Self {
        ArgDecodeInput {
            api,
            arg_index,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> TopDecodeInput for ArgDecodeInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    fn byte_len(&self) -> usize {
        self.api.get_argument_len(self.arg_index)
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.get_argument_boxed_slice_u8(self.arg_index)
    }

    fn into_u64(self) -> u64 {
        self.api.get_argument_u64(self.arg_index)
    }

    fn into_i64(self) -> i64 {
        self.api.get_argument_i64(self.arg_index)
    }
}
