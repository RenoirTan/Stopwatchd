use std::io::{stdin, Read, self, stdout, Write};

use uuid::Uuid;

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