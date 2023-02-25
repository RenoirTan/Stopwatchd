use std::io::{stdin, Read, self, stdout, Write};

use uuid::Uuid;

pub fn press_enter_to_continue() -> io::Result<()> {
    print!("Press enter to continue> ");
    stdout().flush()?;
    stdin().read(&mut [0])?;
    Ok(())
}

pub fn uuid_like_identifier(uuid: &Uuid, test: &str) -> bool {
    // Remove hyphens and make it lower case
    let test = test.replace("-", "").to_lowercase();
    uuid.simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .starts_with(&test)
}