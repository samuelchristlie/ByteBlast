use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::io;

fn main() {
	let mut gen = rand::thread_rng();
	let mut input = String::new();

	println!("\n\n    ____        __       ____  __           __ \n   / __ )__  __/ /____  / __ )/ /___ ______/ /_\n  / __  / / / / __/ _ \\/ __  / / __ `/ ___/ __/\n / /_/ / /_/ / /_/  __/ /_/ / / /_/ (__  ) /_  \n/_____/\\__, /\\__/\\___/_____/_/\\__,_/____/\\__/  \n      /____/                                   \n\n");
	println!("[https://github.com/samuelchristlie/ByteBlast]");

	print!("[?] Enter filename: ");
	io::stdout().flush();
	io::stdin().read_line(&mut input);
	let filename = input.clone();
	input.clear();

	print!("[?] Enter byte size: ");
	io::stdout().flush();
	io::stdin().read_line(&mut input);
	let size = input.trim().parse::<i32>().unwrap() as usize;
	input.clear();

	let batchSize = 4194304;
	let mut bytes = vec![0u8; batchSize];
	let mut file = File::create(filename.trim()).unwrap();

	for _ in 0..(size/batchSize){
		gen.fill(&mut bytes[..]);

		file.write_all(&bytes).unwrap();
	}

	let mut lastBytes = vec![0u8; size % batchSize];
	gen.fill(&mut lastBytes[..]);

	file.write_all(&lastBytes).unwrap();

	

	println!("[!] File successfully created!");
}