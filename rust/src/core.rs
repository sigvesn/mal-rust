use types::*;
use printer::pr_str;
use reader::read_str;
use env::Env;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

lazy_static! {
    pub static ref NS: HashMap<String, fn(&mut Vec<MalType>, Option<Env>) -> MalResult> = {
        let mut ns: HashMap<String, fn(&mut Vec<MalType>, Option<Env>) -> MalResult> = HashMap::new();
        ns.insert("+".to_string(), add);
        ns.insert("-".to_string(), subtract);
        ns.insert("*".to_string(), multiply);
        ns.insert("/".to_string(), divide);
        ns.insert("prn".to_string(), prn);
        ns.insert("println".to_string(), println_fn);
        ns.insert("str".to_string(), str_fn);
        ns.insert("pr-str".to_string(), pr_str_fn);
        ns.insert("list".to_string(), list);
        ns.insert("list?".to_string(), is_list);
        ns.insert("empty?".to_string(), is_empty);
        ns.insert("count".to_string(), count);
        ns.insert("=".to_string(), is_equal);
        ns.insert("<".to_string(), is_lt);
        ns.insert("<=".to_string(), is_lte);
        ns.insert(">".to_string(), is_gt);
        ns.insert(">=".to_string(), is_gte);
        ns.insert("not".to_string(), not);
        ns.insert("read-string".to_string(), read_string);
        ns.insert("slurp".to_string(), slurp);
        ns.insert("atom".to_string(), atom);
        ns.insert("atom?".to_string(), is_atom);
        ns.insert("deref".to_string(), deref);
        ns.insert("reset!".to_string(), reset);
        ns.insert("swap!".to_string(), swap);
        ns.insert("cons".to_string(), cons);
        ns.insert("concat".to_string(), concat);
        ns.insert("nth".to_string(), nth);
        ns.insert("first".to_string(), first);
        ns.insert("rest".to_string(), rest);
        ns
    };
}

pub fn add(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            answer += num?;
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one number".to_string(),
                ))
    }
}

pub fn subtract(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            answer -= num?;
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one number".to_string(),
                ))
    }
}

pub fn multiply(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            answer *= num?;
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one number".to_string(),
                ))
    }
}

pub fn divide(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            let num = num?;
            if num == 0 {
                return Err(MalError::DivideByZero);
            } else {
                answer /= num;
            }
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one number".to_string(),
                ))
    }
}

fn _println(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    let out = results.join(joiner);
    println!("{}", out);
    Ok(MalType::Nil)
}

fn println_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _println(args, false, " ")
}

fn prn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _println(args, true, " ")
}

fn _str_fn(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    Ok(MalType::String(results.join(joiner)))
}

fn str_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _str_fn(args, false, "")
}

fn pr_str_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _str_fn(args, true, " ")
}

fn list(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    Ok(MalType::List(args.clone()))
}

fn mal_bool(b: bool) -> MalType {
    if b {
        MalType::True
    } else {
        MalType::False
    }
}

fn is_list(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        if let MalType::List(_) = arg {
            Ok(MalType::True)
        } else {
            Ok(MalType::False)
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to list?".to_string(),
                ))
    }
}

fn is_empty(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        let vec = raw_vec(&arg)?;
        Ok(mal_bool(vec.len() == 0))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to empty?".to_string(),
                ))
    }
}

fn count(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        let vec = raw_vec(&arg)?;
        Ok(MalType::Number(vec.len() as i64))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to count".to_string(),
                ))
    }
}

fn is_list_like(val: &MalType) -> bool {
    match val {
        &MalType::List(_) | &MalType::Vector(_) => true,
        _ => false,
    }
}

fn are_lists_equal(list1: &MalType, list2: &MalType) -> bool {
    match (list1, list2) {
        (&MalType::List(ref vec1), &MalType::List(ref vec2))
            | (&MalType::List(ref vec1), &MalType::Vector(ref vec2))
            | (&MalType::Vector(ref vec1), &MalType::List(ref vec2))
            | (&MalType::Vector(ref vec1), &MalType::Vector(ref vec2)) => {
                if vec1.len() == vec2.len() {
                    for (index, item1) in vec1.iter().enumerate() {
                        let item2 = &vec2[index];
                        if !is_equal_bool(item1, item2) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        _ => false,
    }
}

fn is_equal_bool(val1: &MalType, val2: &MalType) -> bool {
    if is_list_like(&val1) && is_list_like(&val2) {
        are_lists_equal(&val1, &val2)
    } else {
        val1 == val2
    }
}

fn is_equal(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() == 2 {
        let arg1 = args.remove(0);
        let arg2 = args.remove(0);
        Ok(mal_bool(is_equal_bool(&arg1, &arg2)))
    } else {
        Err(MalError::WrongArguments(
                "Must pass exactly two arguments to =".to_string(),
                ))
    }
}

fn num_compare(args: &mut Vec<MalType>, compare: &Fn(i64, i64) -> bool) -> MalResult {
    if args.len() == 2 {
        let n1 = raw_num(&args.remove(0))?;
        let n2 = raw_num(&args.remove(0))?;
        Ok(mal_bool(compare(n1, n2)))
    } else {
        Err(MalError::WrongArguments(
                "Must pass exactly two arguments to compare".to_string(),
                ))
    }
}

fn is_lt(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 < n2)
}

fn is_lte(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 <= n2)
}

fn is_gt(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 > n2)
}

fn is_gte(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 >= n2)
}

fn not(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        Ok(match &arg {
            &MalType::False => MalType::True,
            _ => MalType::False,
        })
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to not".to_string(),
                ))
    }
}

fn read_string(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::String(code) = args.remove(0) {
            read_str(&code)
        } else {
            Err(MalError::WrongArguments(
                    "Must pass a string to read_string".to_string(),
                    ))
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to read_string".to_string(),
                ))
    }
}

fn slurp(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::String(path) = args.remove(0) {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(MalType::String(contents))
        } else {
            Err(MalError::WrongArguments(
                    "Must pass a string to slurp".to_string(),
                    ))
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to slurp".to_string(),
                ))
    }
}

fn atom(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        Ok(MalType::Atom(Rc::new(RefCell::new(args.remove(0)))))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to atom".to_string(),
                ))
    }
}

fn is_atom(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::Atom(_) = args.remove(0) {
            Ok(mal_bool(true))
        } else {
            Ok(mal_bool(false))
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to atom?".to_string(),
                ))
    }
}

fn deref(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::Atom(val) = args.remove(0) {
            Ok(val.borrow().clone())
        } else {
            Err(MalError::WrongArguments(
                    "Must pass an atom to deref".to_string(),
                    ))
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to deref".to_string(),
                ))
    }
}

fn reset(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 1 {
        let mut atom = args.remove(0);
        let new_val = args.remove(0);
        if let MalType::Atom(ref mut val) = atom {
            val.replace(new_val.clone());
            Ok(new_val)
        } else {
            Err(MalError::WrongArguments(
                    "Must pass an atom to reset".to_string(),
                    ))
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least two arguments to reset!".to_string(),
                ))
    }
}

fn swap(mut args: &mut Vec<MalType>, env: Option<Env>) -> MalResult {
    let top_env = env.expect("Expected Env passed to swap fn");
    if args.len() > 1 {
        let mut atom = args.remove(0);
        let func = args.remove(0);
        if let MalType::Atom(ref mut val) = atom {
            args.insert(0, val.borrow().to_owned());
            if let Ok(MalType::Function(eval_fn, _)) = top_env.get("eval") {
                match func {
                    MalType::Lambda { env, args: binds, mut body, .. } => {
                        let env = Env::with_binds(Some(&env), binds, args.to_owned());
                        let expr = body.remove(0);
                        let mut eval_args = vec![expr];
                        let new_val = eval_fn(&mut eval_args, Some(env))?;
                        val.replace(new_val.clone());
                        Ok(new_val)
                    },
                    MalType::Function(func, env) => {
                        let new_val = func(&mut args, env)?;
                        val.replace(new_val.clone());
                        Ok(new_val)
                    },
                    _ => {
                        Err(MalError::WrongArguments(
                                "Must pass a function to reset".to_string(),
                                ))
                    }
                }
            } else {
                panic!("eval not a function!");
            }
        } else {
            Err(MalError::WrongArguments(
                    "Must pass an atom to reset".to_string(),
                    ))
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least two arguments to swap!".to_string(),
                ))
    }
}

fn cons(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let item = args.remove(0);
        let list = args.remove(0);
        let mut vec = raw_vec(&list)?;
        vec.insert(0, item);
        Ok(MalType::List(vec))
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least two arguments to cons".to_string(),
                ))
    }
}

fn concat(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let mut result = vec![];
    while args.len() > 0 {
        let vec = raw_vec(&args.remove(0))?;
        for item in vec {
            result.push(item);
        }
    }
    Ok(MalType::List(result))
}

fn nth(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let list = args.remove(0);
        let index = raw_num(&args.remove(0))? as usize;
        let vec = raw_vec(&list)?;
        if vec.len() > index {
            Ok(vec[index].clone())
        } else {
            Err(MalError::IndexOutOfBounds { size: vec.len(), index })
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least two arguments to nth".to_string(),
                ))
    }
}

fn first(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let list = args.remove(0);
        match list {
            MalType::List(vec) | MalType::Vector(vec) => {
                if vec.len() > 0 {
                    Ok(vec[0].clone())
                } else {
                    Ok(MalType::Nil)
                }
            }
            MalType::Nil => Ok(MalType::Nil),
            _ => {
                Err(MalError::WrongArguments(
                        format!("Expected a list passed to first but got: {:?}", list).to_string(),
                        ))
            }
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to first".to_string(),
                ))
    }
}

fn rest(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let list = args.remove(0);
        match list {
            MalType::List(mut vec) | MalType::Vector(mut vec) => {
                if vec.len() > 0 {
                    vec.remove(0);
                    Ok(MalType::List(vec))
                } else {
                    Ok(MalType::List(vec![]))
                }
            }
            MalType::Nil => Ok(MalType::List(vec![])),
            _ => {
                Err(MalError::WrongArguments(
                        format!("Expected a list passed to rest but got: {:?}", list).to_string(),
                        ))
            }
        }
    } else {
        Err(MalError::WrongArguments(
                "Must pass at least one argument to rest".to_string(),
                ))
    }
}

fn raw_num(arg: &MalType) -> Result<i64, MalError> {
    if let MalType::Number(num) = *arg {
        Ok(num)
    } else {
        Err(MalError::WrongArguments(
                format!("Expected a number but got: {:?}", arg).to_string()
                ))
    }
}

fn raw_vec(arg: &MalType) -> Result<Vec<MalType>, MalError> {
    match *arg {
        MalType::List(ref vec) | MalType::Vector(ref vec) => Ok(vec.clone()),
        _ => Err(MalError::WrongArguments(
                format!("Expected a list or vector but got: {:?}", arg).to_string()
                ))
    }
}

struct MalNumberIter<'a> {
    items: &'a mut Vec<MalType>,
}

impl<'a> Iterator for MalNumberIter<'a> {
    type Item = Result<i64, MalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.len() == 0 {
            return None;
        }
        let item = self.items.remove(0);
        if let MalType::Number(num) = item {
            Some(Ok(num))
        } else {
            Some(Err(MalError::NotANumber))
        }
    }
}
