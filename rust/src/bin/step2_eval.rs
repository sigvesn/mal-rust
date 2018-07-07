extern crate mal_rust;

use mal_rust::core;
use mal_rust::printer::pr_str;
use mal_rust::reader::read_str;
use mal_rust::readline::Readline;
use mal_rust::types::*;

use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let mut readline = Readline::new("user> ");
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    let result = rep(line);
                    match result {
                        Ok(str) => println!("{}", str),
                        Err(MalError::BlankLine) => {}
                        Err(err) => println!("{}", err),
                    }
                }
            }
            None => break,
        }
    }
    readline.save_history();
}

type ReplEnv = HashMap<String, MalType>;

fn rep(input: String) -> Result<String, MalError> {
    let mut repl_env: ReplEnv = HashMap::new();
    repl_env.insert(
        "+".to_string(),
        MalType::function(Box::new(core::add), None),
    );
    repl_env.insert(
        "-".to_string(),
        MalType::function(Box::new(core::subtract), None),
    );
    repl_env.insert(
        "*".to_string(),
        MalType::function(Box::new(core::multiply), None),
    );
    repl_env.insert(
        "/".to_string(),
        MalType::function(Box::new(core::divide), None),
    );
    let out = read(input)?;
    let out = eval(out, &repl_env)?;
    let out = print(out);
    Ok(out)
}

fn read(code: String) -> MalResult {
    read_str(&code)
}

fn eval(ast: MalType, repl_env: &ReplEnv) -> MalResult {
    match ast {
        MalType::List(_, _) => {
            if list_len(&ast) == 0 {
                Ok(ast)
            } else {
                let new_ast = eval_ast(ast, repl_env)?;
                if let MalType::List(mut vec, _) = new_ast {
                    if vec.len() > 0 {
                        let first = vec.remove(0);
                        if let MalType::Function { func, .. } = first {
                            func(&mut vec, None)
                        } else {
                            Err(MalError::NotAFunction(first))
                        }
                    } else {
                        panic!("Eval'd list is empty!")
                    }
                } else {
                    panic!("Eval'd list is no longer a list!")
                }
            }
        }
        _ => Ok(eval_ast(ast, repl_env)?),
    }
}

fn print(ast: MalType) -> String {
    pr_str(&ast, true)
}

fn eval_ast(ast: MalType, repl_env: &ReplEnv) -> MalResult {
    match ast {
        MalType::Symbol(symbol) => {
            if let Some(val) = repl_env.get(&symbol) {
                Ok(val.to_owned())
            } else {
                Err(MalError::SymbolUndefined(symbol.to_string()))
            }
        }
        MalType::List(vec, _) => {
            let results: Result<Vec<MalType>, MalError> =
                vec.into_iter().map(|item| eval(item, repl_env)).collect();
            Ok(MalType::list(results?))
        }
        MalType::Vector(vec, _) => {
            let results: Result<Vec<MalType>, MalError> =
                vec.into_iter().map(|item| eval(item, repl_env)).collect();
            Ok(MalType::vector(results?))
        }
        MalType::HashMap(map, metadata) => {
            let mut new_map = BTreeMap::new();
            for (key, val) in map {
                new_map.insert(key, eval(val, repl_env)?);
            }
            Ok(MalType::HashMap(new_map, metadata))
        }
        _ => Ok(ast),
    }
}

fn list_len(list: &MalType) -> usize {
    if let &MalType::List(ref vec, _) = list {
        vec.len()
    } else {
        panic!("Not a list!")
    }
}