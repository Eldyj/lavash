use std::fs;
#[path = "compiler.rs"]
mod compiler;

pub fn eval(code: String) {
    print!("{}", compiler::compile(code));
}

pub fn print(file: String) {
    let file_content: String = fs::read_to_string(file).expect("error while reading file");
    print!("{}", compiler::compile(file_content));
}

pub fn build(file: String, output: String) {
    let file_content: String = fs::read_to_string(file).expect("error while reading file");
    let result: String = compiler::compile(file_content);
    fs::write(output, result).expect("error while writing file");
}

pub fn help() {
    let arguments: Vec<&str> = vec!["--print|-p", "--eval|-e", "--version|-v", "--help|-h"];

    let descriptions: Vec<&str> = vec![
        "just show result of file compilation without replacing existing spwn file",
        "compile given string and print result",
        "print version of preprocessor",
        "show this message",
    ];

    println!("lavash - spwn preprocessor");
    println!("usage lavash [arguments] [file] [output file]");
    println!("------------------");
    for i in 0..arguments.len() {
        println!("{} - {}", arguments[i], descriptions[i]);
    }
}

pub fn version() {
    println!("lavash v0.1 alpha");
}
