use crate::{Parameters, PrimeField};
use derive_more::{Display, Error, From};
use std::marker::PhantomData;
use zkp_macros_decl::u256h;

use zkp_u256::{to_montgomery_const, U256};

// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
pub type FieldElement = PrimeField<Proth>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Proth();

#[derive(Debug, Display, Error, From)]
pub enum PrimeFieldError {
    HexError(hex::FromHexError),
}

impl Parameters for Proth {
    type UInt = U256;

    /// 3, in montgomery form.
    const GENERATOR: U256 =
        u256h!("07fffffffffff9b0ffffffffffffffffffffffffffffffffffffffffffffffa1");
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    ///
    const ORDER: U256 = u256h!("0800000000000011000000000000000000000000000000000000000000000000");
    const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
    const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
    const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
}

impl FieldElement {
    /// Creates a constant value from a `U256` constant in Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_montgomery_const(uint: U256) -> Self {
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    /// Creates a constant value from a `U256` constant.
    ///
    /// It does compile-time conversion to Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_uint_const(n: &U256) -> Self {
        let uint = to_montgomery_const(n, &Proth::MODULUS, Proth::M64, &Proth::R2);
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    pub fn from_hex_str(hex_string: &str) -> Result<Self, PrimeFieldError> {
        let uint = <[u8; 32] as hex::FromHex>::from_hex(hex_string).map(|r| {
            let mut res: [u64; 4] = [0; 4];
            for i in 0..res.len() {
                for j in 3..=0 {
                    let tmp: u64 = res[i] << 8;
                    let tmp2: u64 = r[i * 4 + j].into();
                    res[i] = tmp + tmp2
                }
            }
            return Self {
                uint: U256::from_limbs(res),
                _parameters: PhantomData,
            };
        })?;
        return Ok(uint);
    }
}
