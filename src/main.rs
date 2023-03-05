use std::fs;
use std::io;
use std::env;
use std::vec::Vec;

fn read_file(path: &String) -> Vec<String> {
	fs::read_to_string(path).expect("Failed to read file")
		.split("\n")
		.map(|x| x.into())
		.collect::<Vec<String>>()
}

fn write_file(path: &String, lines: &Vec<String>) {
	_ = fs::write(path, lines.join("\n"));
}

fn input() -> String {
	let mut out: String = "".into();
	io::stdin().read_line(&mut out).ok().expect("Failed to read line");
	out.chars().filter(|&x| x != '\n').collect()
}

#[derive(PartialEq)]
enum Mode {
	Append,
	Command,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Not enough arguments. it should be 'med [file]'");
		std::process::exit(1);
	}
	
	let path: &String = &args[1];
	
    let mut lines = read_file(&path);

	let mut cursor = lines.len()-1;
	let mut inp: String = "".into();
	let mut mode: Mode = Mode::Command;
	
	while inp != "q" {
		match inp.as_str() {
			"a" => mode = Mode::Append,
			"." => mode = Mode::Command,
			"d" => if mode == Mode::Command {
					lines.remove(cursor);
			},
			"-" => if mode == Mode::Command
				&& cursor > 0 {
					cursor -= 1;
					println!("Line: {}", lines[cursor]);
			},
			"+" => if mode == Mode::Command
				&& cursor < lines.len()-1 {
					cursor += 1;
					println!("Line: {}", lines[cursor]);
			},
			"p" => if mode == Mode::Command {
				println!("Line: {}", lines[cursor]);
			},
			"P" => {
				println!("-----");
				for line in lines.iter() {
					println!("{}", line);
				}
				println!("-----");
			},
			"w" => {
				write_file(&path, &lines);
			}
			_ => {
				if mode == Mode::Append {
					lines.insert(cursor, inp);
				} else if inp.len() > 0 {
					println!("Unknown Command: '{}'", inp);
				}
			},
		};

		inp = input();
	}
}
