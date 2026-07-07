pub mod arithmetics;
pub mod string_manip;

use std::error::Error;
use std::io;

type CommandHandler = fn(&[String]) -> Result<String, Box<dyn Error>>;

#[derive(Clone, Copy)]
enum Arity {
    Exact(usize),
    AtLeast(usize),
}

struct CommandSpec {
    name: &'static str,
    arity: Arity,
    handler: CommandHandler,
}

fn invalid_input(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::new(io::ErrorKind::InvalidInput, message.into()))
}

fn command_registry() -> &'static [CommandSpec] {
    &[
        CommandSpec {
            name: "add_two_ints",
            arity: Arity::Exact(2),
            handler: handle_add_two_ints,
        },
        CommandSpec {
            name: "reverse_string",
            arity: Arity::Exact(1),
            handler: handle_reverse_string,
        },
        CommandSpec {
            name: "syllable_count_in_hyphenated_word",
            arity: Arity::Exact(1),
            handler: handle_syllable_count_in_hyphenated_word,
        },
        CommandSpec {
            name: "absolute_sum_of_int_list",
            arity: Arity::AtLeast(1),
            handler: handle_absolute_sum_of_int_list,
        },
        CommandSpec {
            name: "burp_nr",
            arity: Arity::Exact(1),
            handler: handle_burp_nr,
        },
        CommandSpec {
            name: "solid_clump_of_hashes",
            arity: Arity::AtLeast(1),
            handler: handle_solid_clump_hashes,
        },
        CommandSpec {
            name: "more_odd_in_list",
            arity: Arity::AtLeast(1),
            handler: handle_more_odd,
        },
    ]
}

fn parse_exact_arity(command: &str, args: &[String], expected: usize) -> Result<(), Box<dyn Error>> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(invalid_input(format!(
            "{command} expects exactly {expected} argument(s)"
        )))
    }
}

fn parse_required_int(command: &str, value: &str, label: &str) -> Result<i32, Box<dyn Error>> {
    value.parse::<i32>().map_err(|error| {
        invalid_input(format!(
            "{command} failed to parse {label} as integer: {error}"
        ))
    })
}

fn parse_required_uint(command: &str, value: &str, label: &str) -> Result<u32, Box<dyn Error>> {
    value.parse::<u32>().map_err(|error| {
        invalid_input(format!(
            "{command} failed to parse {label} as integer: {error}"
        ))
    })
}

fn dispatch(command: &str, args: &[String]) -> Result<String, Box<dyn Error>> {
    let spec = command_registry()
        .iter()
        .find(|entry| entry.name == command)
        .ok_or_else(|| invalid_input(format!("unknown command: {command}")))?;

    match spec.arity {
        Arity::Exact(expected) => parse_exact_arity(spec.name, args, expected)?,
        Arity::AtLeast(minimum) if args.len() < minimum => {
            return Err(invalid_input(format!(
                "{command} expects at least {minimum} argument(s)"
            )));
        }
        Arity::AtLeast(_) => {}
    }

    (spec.handler)(args)
}


// region: -- handler functions -------------------------------------------------------------------

fn handle_add_two_ints(args: &[String]) -> Result<String, Box<dyn Error>> {
    let left = parse_required_int("add_two_ints", &args[0], "left argument")?;
    let right = parse_required_int("add_two_ints", &args[1], "right argument")?;

    Ok(arithmetics::add_two_ints(left, right).to_string())
}

fn handle_reverse_string(args: &[String]) -> Result<String, Box<dyn Error>> {
    Ok(string_manip::reverse_string(&args[0]))
}

fn handle_syllable_count_in_hyphenated_word(args: &[String]) -> Result<String, Box<dyn Error>> {
    Ok(string_manip::syllable_count_in_hyphenated_word(&args[0]).to_string())
}

fn handle_absolute_sum_of_int_list(args: &[String]) -> Result<String, Box<dyn Error>> {
    let values = args
        .iter()
        .enumerate()
        .map(|(index, value)| {
            parse_required_int("absolute_sum_of_int_list", value, &format!("argument {index}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(arithmetics::absolute_sum_of_int_list(&values).to_string())
}

fn handle_burp_nr (args: &[String]) -> Result<String, Box<dyn Error>> {
    let r_count = parse_required_uint ("Burp_nr", &args[0], "r_count")? as usize;
    Ok(string_manip::burp_nr(r_count))
}

fn handle_solid_clump_hashes (args: &[String]) -> Result<String, Box<dyn Error>> {
    Ok(string_manip::solid_clump_of_hashes(&args.join("")).to_string())
}

fn handle_more_odd (args: &[String]) -> Result<String, Box<dyn Error>> {
    let values = args
        .iter()
        .enumerate()
        .map(|(index, value)| {
            parse_required_int("more_odd_in_list", value, &format!("argument {index}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(arithmetics::more_odd_in_list(&values).to_string())
}

// endregion: handler functions -------------------------------------------------------------------


pub fn runner(args: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let mut args = args;
    let command = args
        .next()
        .ok_or_else(|| invalid_input("no command provided"))?;
    let remaining_args: Vec<String> = args.collect();

    dispatch(&command, &remaining_args)
}

