use std::{io::{stdin, Read, self, stdout, Write}, collections::HashMap};

use uuid::Uuid;

use crate::identifiers::Identifier;

pub fn press_enter_to_continue() -> io::Result<()> {
    print!("Press enter to continue> ");
    stdout().flush()?;
    stdin().read(&mut [0])?;
    Ok(())
}

pub const UUID_STRLEN: usize = 32;

#[inline]
pub fn get_uuid_node(uuid: &Uuid) -> u64 {
    uuid.as_u64_pair().1 & ((1 << 48) - 1)
}

#[inline]
pub fn uuid_is_identifier(uuid: &Uuid, test: &str) -> bool {
    uuid_like_identifier(uuid, test) == UUID_STRLEN
}

/// Return how many of the first several characters match `test`
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