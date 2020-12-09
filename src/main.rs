#![feature(or_patterns, in_band_lifetimes, drain_filter, bool_to_option)]

mod code;
mod compiler;
mod eval;

use std::{path::PathBuf, process};

use eval::{env::Env, Object};
use log::error;
use parser::{parse_stdin, parse_str, NodePtr};
use process::exit;
use rustyline::{error::ReadlineError, Editor};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "minus2c",
    about = "An implementation of the --C language in rust-lang."
)]
struct Opt {
    #[structopt(short, long)]
    print_tree: bool,

    /// Runs evaluation of given minus2c code.
    #[structopt(short, long)]
    eval: bool,

    /// Run in Repl
    #[structopt(long)]
    repl: bool,

    /// Input file, default to stdin.
    #[structopt(parse(from_os_str), required_if("repl", "false"))]
    input: Option<PathBuf>,

    #[structopt(short)]
    output: Option<PathBuf>,
}

fn repl() {
    // start repl
    println!(r#"Welcome to minus2c interpreter!"#);

    let mut rl = Editor::<()>::new();
    let mut env = Env::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(line.as_str());

                    match eval::eval_repl(line.as_str(), &mut env) {
                        Ok(result) => {
                            if result != Object::Void {
                                println!("{}", result)
                            }
                        }

                        Err(err) => println!("{}", err),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn main() {
    colog::init();
    let opt = Opt::from_args();

    if opt.repl {
        repl()
    } else if let Some(source_tree) = read_source_code(opt.input) {
        // debug print tree
        if opt.print_tree {
            unsafe { parser::print_tree(source_tree) };
        }

        if opt.eval {
            let rs = eval::eval_source_tree(source_tree);
            let exit_code = match rs {
                Ok(Object::Int(return_code)) => return_code,
                Ok(Object::Void) => 0,
                Ok(_) => {
                    error!("minus2c: main returned non-int type");
                    1
                }

                Err(err) => {
                    error!("{}", err);
                    1
                }
            };

            exit(exit_code);
        } else {
            compiler::compile(source_tree);
        }
    } else {
        exit(1);
    }
}

fn read_source_code(input_path: Option<PathBuf>) -> Option<NodePtr> {
    if let Some(input_path) = input_path {
        if !input_path.is_file() {
            println!("minus2c: no such file {:?}", input_path);
            None
        } else {
            let source =
                std::fs::read_to_string(input_path).expect("unable to read file contents.");

            parse_str(source.as_str())
        }
    } else {
        unsafe { parse_stdin() }
    }
}
