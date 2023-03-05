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
			"P" => if mode == Mode::Command {
				println!("-----");
				for line in lines.iter() {
					println!("{}", line);
				}
				println!("-----");
			},
			"n" => if mode == Mode::Command{
				lines.push("".into());
			},
			"w" => {
				write_file(&path, &lines);
			},
			"h" => {
				println!(
					"{}\n{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n-----",
					"Usage: 'med [file]' (you cant create a file using med, it has to already exist)",
					"Help Menu:",
					"'a' - start Append Mode, start writing text",
					"'.' - go back to Command Mode",
					"'d' - delete line at cursor",
					"'n' - insert newline at the end of the file",
					"'-' - move cursor down",
					"'+' - move cursor up",
					"'p' - print line at cursor",
					"'P' - print entire file",
					"'w' - write to file",
					"'q' - quit med",
					"'h' - show this menu"
				);
			}
			_ => {
				if mode == Mode::Append {
					lines.insert(cursor, inp);
					cursor += 1;
				} else if inp.len() > 0 {
					println!("Unknown Command: '{}'", inp);
				}
			},
		};

		inp = input();
	}
}
