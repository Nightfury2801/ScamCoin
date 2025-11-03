use std::io;
use std::fs::File;
use io::Read;
use std::path::Path;

/*
Note that file locks don't stop anyone who doesn't care about the locks from
reading or writing data to the file, it just stops programs that do implement
the lock.
*/

const CHAIN_PATH: &str = "./scamcoin.chain";

fn main() -> Result<(), io::Error> {
	// check if file is there. open if it is
	if !Path::new(CHAIN_PATH).is_file() {
		File::create(CHAIN_PATH)
			.expect("Couldn't create blockchain file\n");
	}
	let mut file = File::open(CHAIN_PATH)
		.expect("Couldn't open file\n");

	// lock the file and read it
	println!("Trying to acquire file lock");
	file.lock()?;

	println!("file locked");

	let mut bytes: [u8; 1024] = [0; 1024];
	let n = file.read(&mut bytes)?;
	println!("read {n} bytes");

	for i in bytes {
		if i == 0 { break }
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
