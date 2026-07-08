pub mod arithmetics;
pub mod string_manip;

use std::error::Error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

type CommandHandler = Box<dyn Fn(&[String]) -> Result<String, Box<dyn Error>>>;

struct CommandSpec {
    name: &'static str,
    handler: CommandHandler,
}

fn invalid_input(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::new(io::ErrorKind::InvalidInput, message.into()))
}

fn parse_arg<T>(command: &str, value: &str, label: &str) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Display,
{
    value
        .parse::<T>()
        .map_err(|error| invalid_input(format!("{command} failed to parse {label}: {error}")))
}

// region: -- shape adapters -----------------------------------------------------------------
// Each adapter captures a command name (for error messages) and a target function, and
// returns a boxed closure matching the uniform CommandHandler signature. Adding a new
// problem with an already-seen parameter shape is a one-line registry entry; only a genuinely
// new shape needs a new adapter here.

fn adapt1<T, R, F>(command: &'static str, f: F) -> CommandHandler
where
    T: FromStr,
    T::Err: Display,
    R: ToString,
    F: Fn(T) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(invalid_input(format!("{command} expects exactly 1 argument")));
        }
        let value = parse_arg::<T>(command, &args[0], "argument")?;
        Ok(f(value).to_string())
    })
}

fn adapt2<T1, T2, R, F>(command: &'static str, f: F) -> CommandHandler
where
    T1: FromStr,
    T1::Err: Display,
    T2: FromStr,
    T2::Err: Display,
    R: ToString,
    F: Fn(T1, T2) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() != 2 {
            return Err(invalid_input(format!("{command} expects exactly 2 arguments")));
        }
        let a = parse_arg::<T1>(command, &args[0], "argument 1")?;
        let b = parse_arg::<T2>(command, &args[1], "argument 2")?;
        Ok(f(a, b).to_string())
    })
}

fn adapt3<T1, T2, T3, R, F>(command: &'static str, f: F) -> CommandHandler
where
    T1: FromStr,
    T1::Err: Display,
    T2: FromStr,
    T2::Err: Display,
    T3: FromStr,
    T3::Err: Display,
    R: ToString,
    F: Fn(T1, T2, T3) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() != 3 {
            return Err(invalid_input(format!("{command} expects exactly 3 arguments")));
        }
        let a = parse_arg::<T1>(command, &args[0], "argument 1")?;
        let b = parse_arg::<T2>(command, &args[1], "argument 2")?;
        let c = parse_arg::<T3>(command, &args[2], "argument 3")?;
        Ok(f(a, b, c).to_string())
    })
}

/// Any number of same-typed args, e.g. `Vec<i32>` or `Vec<u32>`.
fn adapt_list<T, R, F>(command: &'static str, f: F) -> CommandHandler
where
    T: FromStr,
    T::Err: Display,
    R: ToString,
    F: Fn(&[T]) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.is_empty() {
            return Err(invalid_input(format!("{command} expects at least 1 argument")));
        }
        let values = args
            .iter()
            .enumerate()
            .map(|(index, value)| parse_arg::<T>(command, value, &format!("argument {index}")))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(f(&values).to_string())
    })
}

/// A single arg taken by reference, e.g. `fn(&str) -> R`. Distinct from `adapt1` because this
/// borrows the arg directly rather than parsing an owned value — parsing a String and handing
/// it to a `fn(&str)` doesn't type-check.
fn adapt_str<R, F>(command: &'static str, f: F) -> CommandHandler
where
    R: ToString,
    F: Fn(&str) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(invalid_input(format!("{command} expects exactly 1 argument")));
        }
        Ok(f(&args[0]).to_string())
    })
}

/// All args joined into a single string before being passed on. `join_with` controls the
/// separator ("" for concatenation, " " for sentence-style joins).
fn adapt_joined<R, F>(command: &'static str, join_with: &'static str, f: F) -> CommandHandler
where
    R: ToString,
    F: Fn(&str) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.is_empty() {
            return Err(invalid_input(format!("{command} expects at least 1 argument")));
        }
        let joined = args.join(join_with);
        Ok(f(&joined).to_string())
    })
}

// endregion: shape adapters -----------------------------------------------------------------

fn command_registry() -> Vec<CommandSpec> {
    vec![
        CommandSpec {
            name: "add_two_ints",
            handler: adapt2("add_two_ints", arithmetics::add_two_ints),
        },
        CommandSpec {
            name: "reverse_string",
            handler: adapt_str("reverse_string", string_manip::reverse_string),
        },
        CommandSpec {
            name: "syllable_count_in_hyphenated_word",
            handler: adapt_str(
                "syllable_count_in_hyphenated_word",
                string_manip::syllable_count_in_hyphenated_word,
            ),
        },
        CommandSpec {
            name: "absolute_sum_of_int_list",
            handler: adapt_list("absolute_sum_of_int_list", arithmetics::absolute_sum_of_int_list),
        },
        CommandSpec {
            name: "burp_nr",
            handler: adapt1("burp_nr", string_manip::burp_nr),
        },
        CommandSpec {
            name: "solid_clump_of_hashes",
            handler: adapt_joined("solid_clump_of_hashes", "", string_manip::solid_clump_of_hashes),
        },
        CommandSpec {
            name: "more_odd_in_list",
            handler: adapt_list("more_odd_in_list", arithmetics::more_odd_in_list),
        },
        CommandSpec {
            name: "count_words_in_sentence",
            handler: adapt_joined(
                "count_words_in_sentence",
                " ",
                string_manip::count_words_in_sentence,
            ),
        },
        CommandSpec {
            name: "count_distinct_quadratic_roots",
            handler: adapt3(
                "count_distinct_quadratic_roots",
                arithmetics::count_distinct_quadratic_roots,
            ),
        },
        CommandSpec {
            name: "last_digit_c_equals_of_ab",
            handler: adapt3(
                "last_digit_c_equals_of_ab",
                arithmetics::last_digit_c_equals_of_ab,
            ),
        },
        CommandSpec {
            name: "positive_descending_pair",
            handler: adapt2("positive_descending_pair", arithmetics::positive_descending_pair),
        },
        CommandSpec {
            name: "vowel_counter",
            handler: adapt_joined("vowel_counter", " ", string_manip::vowel_counter),
        },
    ]
}

fn dispatch(command: &str, args: &[String]) -> Result<String, Box<dyn Error>> {
    let spec = command_registry()
        .into_iter()
        .find(|entry| entry.name == command)
        .ok_or_else(|| invalid_input(format!("unknown command: {command}")))?;

    (spec.handler)(args)
}

pub fn runner(args: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let mut args = args;
    let command = args
        .next()
        .ok_or_else(|| invalid_input("no command provided"))?;
    let remaining_args: Vec<String> = args.collect();

    dispatch(&command, &remaining_args)
}
