use std::fs::File;
use std::io;
use std::path::Path;

/*
Note that file locks don't stop anyone who doesn't care about the locks from
reading or writing data to the file, it just stops programs that do implement
the lock.
*/

const CHAIN_PATH: &str = "/tmp/scamcoin.chain";

fn main() -> Result<(), std::io::Error> {
	// check if file is there. open if it is
	let file = if !Path::new(CHAIN_PATH).is_file() {
		File::create(CHAIN_PATH)
			.expect("Couldn't create blockchain file\n")
	}
	else {
		File::open(CHAIN_PATH)
			.expect("Couldn't open file\n")
	};

	// lock the file and read it
	println!("Trying to acquire file lock");
	file.lock()?;

	println!("file locked");
	let bytes: Vec<u8> = std::fs::read(CHAIN_PATH)?;

	for i in bytes {
		println!("Read bytes: {}", i);
	}

	// block for lock testing
	println!("hit enter to continue");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)?;

	// unlock file
	file.unlock()?;
	println!("file unlocked");
	Ok(())
}
