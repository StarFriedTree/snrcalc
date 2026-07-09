pub mod arithmetics;
pub mod string_manip;
pub mod array_manip;

use std::error::Error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

type CommandHandler = Box<dyn Fn(&[String]) -> Result<String, Box<dyn Error>>>;

struct CommandSpec {
    name: &'static str,
    handler: CommandHandler,
    if_fails: &'static str,
}

/// Registry entry with no hint (the common case).
fn spec(name: &'static str, handler: CommandHandler) -> CommandSpec {
    CommandSpec { name, handler, if_fails: "" }
}

/// Registry entry with a hint appended to the error message on failure.
fn spec_hinted(name: &'static str, handler: CommandHandler, if_fails: &'static str) -> CommandSpec {
    CommandSpec { name, handler, if_fails }
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

fn adapt_1_list<T1, T2, R, F> (command: &'static str, f: F) -> CommandHandler
where
    T1: FromStr,
    T1::Err: Display,
    T2: FromStr,
    T2::Err: Display,
    R: ToString,
    F: Fn(T1, &[T2]) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.is_empty() {
            return Err(invalid_input(format!("{command} expects at least 2 arguments")));
        }
        let item = parse_arg::<T1>(command, &args[0], "argument 1")?;
        let values = args[1..]
            .iter()
            .enumerate()
            .map(|(index, value)| parse_arg::<T2>(command, value, &format!("argument {index}")))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(f(item, &values).to_string())
    })
}

/// Any number of borrowed string args, for functions that operate on `&[&str]` directly.
fn adapt_str_list<R, F>(command: &'static str, f: F) -> CommandHandler
where
    R: ToString,
    F: Fn(&[&str]) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.is_empty() {
            return Err(invalid_input(format!("{command} expects at least 1 argument")));
        }
        let values: Vec<&str> = args.iter().map(String::as_str).collect();
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

fn adapt_str_char<R, F>(command: &'static str, f: F) -> CommandHandler
where
    R: ToString,
    F: Fn(&str, &char) -> R + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() != 2 {
            return Err(invalid_input(format!("{command} expects exactly 2 arguments")));
        }
        let replacement = parse_arg::<char>(command, &args[1], "argument 2")?;
        Ok(f(&args[0], &replacement).to_string())
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
        spec("add_two_ints", adapt2("add_two_ints", arithmetics::add_two_ints)),
        spec("reverse_string", adapt_str("reverse_string", string_manip::reverse_string)),
        spec(
            "syllable_count_in_hyphenated_word",
            adapt_str(
                "syllable_count_in_hyphenated_word",
                string_manip::syllable_count_in_hyphenated_word,
            ),
        ),
        spec(
            "absolute_sum_of_int_list",
            adapt_list("absolute_sum_of_int_list", arithmetics::absolute_sum_of_int_list),
        ),
        spec("burp_nr", adapt1("burp_nr", string_manip::burp_nr)),
        spec(
            "solid_clump_of_hashes",
            adapt_joined("solid_clump_of_hashes", "", string_manip::solid_clump_of_hashes),
        ),
        spec(
            "more_odd_in_list",
            adapt_list("more_odd_in_list", arithmetics::more_odd_in_list),
        ),
        spec(
            "count_words_in_sentence",
            adapt_joined("count_words_in_sentence", " ", string_manip::count_words_in_sentence),
        ),
        spec(
            "count_distinct_quadratic_roots",
            adapt3(
                "count_distinct_quadratic_roots",
                arithmetics::count_distinct_quadratic_roots,
            ),
        ),
        spec(
            "last_digit_c_equals_of_ab",
            adapt3("last_digit_c_equals_of_ab", arithmetics::last_digit_c_equals_of_ab),
        ),
        spec_hinted(
            "positive_descending_pair",
            adapt2("positive_descending_pair", arithmetics::positive_descending_pair),
            "both arguments need to be positive integers",
        ),
        spec("vowel_counter", adapt_joined("vowel_counter", " ", string_manip::vowel_counter)),
        spec("repeat_each_char", adapt_joined("repeat_each_char", " ", string_manip::repeat_each_char)),
        spec(
            "max_pair_difference_in_list", 
            adapt_list("max_pair_difference_in_list", arithmetics::max_pair_difference_in_list)
        ),
        spec_hinted(
            "sum_of_first_n_positive_ints", 
            adapt1("sum_of_first_n_positive_ints", arithmetics::sum_of_first_n_positive_ints),
            "expected a single positive integer",
        ),
        spec(
            "positive_even_negative_odd", 
            adapt1("positive_even_negative_odd", arithmetics::positive_even_negative_odd)
        ),
        spec(
            "count_negatives_in_list", 
            adapt_list("count_negatives_in_list", arithmetics::count_negatives_in_list)
        ),
        spec("sum_of_even_in_list", adapt_list("sum_of_even_in_list", arithmetics::sum_of_even_in_list)),
        spec(
            "multiplication_table_to_ten", 
            adapt1("multiplication_table_to_ten", arithmetics::multiplication_table_to_ten)
        ),
        spec("terminals_of_list", adapt_list("terminals_of_list", arithmetics::terminals_of_list)),
        spec("is_alpha_only", adapt_joined("is_alpha_only", " ", string_manip::is_alpha_only)),
        spec("count_three_equal", adapt3("count_three_equal", arithmetics::count_three_equal)),
        spec("multiplication_table_to_n", adapt2("multiplication_table_to_n", arithmetics::multiplication_table_to_n)),
        spec("replace_vowels", adapt_str_char("replace_vowels", string_manip::replace_vowels)),
        spec("shift_cipher_one", adapt_joined("shift_cipher_one", " ", string_manip::shift_cipher_one)),
        spec("check_palindrome", adapt_joined("check_palindrome", " ", string_manip::check_palindrome)),
        spec(
            "filter_4letter_words_from_list",
            adapt_str_list("filter_4letter_words_from_list", string_manip::filter_4letter_words_from_list),
        ),
        spec("same_amount_of_x_o", adapt_joined("same_amount_of_x_o", " ", string_manip::same_amount_of_x_o)),
        spec(
            "search_int_or_potential_index_in_sorted",
            adapt_1_list("search_int_or_potential_index_in_sorted", array_manip::search_int_or_potential_index_in_sorted),
        ),
        spec(
            "longest_word_in_sentence",
            adapt_joined("longest_word_in_sentence", " ", string_manip::longest_word_in_sentence),
        ),

    ]
}

fn dispatch(command: &str, args: &[String]) -> Result<String, Box<dyn Error>> {
    let spec = command_registry()
        .into_iter()
        .find(|entry| entry.name == command)
        .ok_or_else(|| invalid_input(format!("unknown command: {command}")))?;

    (spec.handler)(args).map_err(|error| {
        if spec.if_fails.is_empty() {
            error
        } else {
            invalid_input(format!("{error}\nNote: {}", spec.if_fails))
        }
    })
}

pub fn runner(args: impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let mut args = args;
    let command = args
        .next()
        .ok_or_else(|| invalid_input("no command provided"))?;
    let remaining_args: Vec<String> = args.collect();

    dispatch(&command, &remaining_args)
}
