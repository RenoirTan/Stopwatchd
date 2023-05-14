//! Utility functions for `stopwatchd`.

use std::{io::{stdin, Read, self, stdout, Write}, collections::HashMap};

use uuid::Uuid;

use crate::identifiers::Identifier;

/// Prompts "Press enter to continue> ".
/// Returns Ok if user presses enter.
pub fn press_enter_to_continue() -> io::Result<()> {
    print!("Press enter to continue> ");
    stdout().flush()?;
    stdin().read(&mut [0])?;
    Ok(())
}

pub const UUID_STRLEN: usize = 32;

/// Get the "node" component in a UUID.
#[inline]
pub fn get_uuid_node(uuid: &Uuid) -> u64 {
    uuid.as_u64_pair().1 & ((1 << 48) - 1)
}

/// Returns whether `uuid`'s string representation matches `test`.
#[warn(deprecated)]
#[inline]
pub fn uuid_is_identifier(uuid: &Uuid, test: &str) -> bool {
    uuid_like_identifier(uuid, test) == UUID_STRLEN
}

/// Return how many of the first several characters of `uuid` match `test`.
#[warn(deprecated)]
pub fn uuid_like_identifier(uuid: &Uuid, test: &str) -> usize {
    // Remove hyphens and make it lower case
    let test = test.replace("-", "").to_lowercase();
    let ok = uuid.simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .starts_with(&test);
    if ok {
        test.len()
    } else {
        0
    }
}

/// Convert an iterator of values (`iter`) into a [`HashMap`] with their corresponding
/// identifiers as keys. You must provide a function `get_identifier` which obtains each
/// value's identifier.
#[warn(deprecated)]
pub fn map_identifier_to_values<I, V, F>(iter: I, mut get_identifier: F) -> HashMap<Identifier, V>
where
    I: IntoIterator<Item = V>,
    F: FnMut(&V) -> Identifier
{
    let mut map = HashMap::new();
    for value in iter {
        let identifier = get_identifier(&value);
        map.insert(identifier, value);
    }
    map
}

/// Collect items of type `T` from `iter` into a [`Vec`] of type `U`.
pub fn iter_into_vec<I, T, U>(iter: I) -> Vec<U>
where
    I: IntoIterator<Item = T>,
    T: Into<U>
{
    iter.into_iter().map(Into::into).collect()
}

/// Implements [`Into`] for data types that are variants of an enum.
/// 
/// Consider the following example for the syntax. The identifier for the
/// enum comes first, then an open '{', and then the variants are declared
/// just like how they were defined in the original `enum Variants { ... }`
/// definition. Once none/some/all of the variants can [`Into`] `Variants`,
/// close the macro input with '}'
/// 
/// # Example
/// 
/// ```
/// enum Variants {
///     A(TypeA),
///     B(TypeB)
/// }
/// 
/// struct TypeA;
/// struct TypeB;
/// 
/// impl_into_enum_variant!(Variants {
///     A(TypeA),
///     B(TypeB)
/// });
/// ```
#[macro_export]
macro_rules! impl_into_enum_variant {
    ( $enumtype:ty { $( $variant:ident($datatype:ty) ),* }) => {
        $(
            impl Into<$enumtype> for $datatype {
                fn into(self) -> $enumtype {
                    <$enumtype>::$variant(self)
                }
            }
        )*
    };
}