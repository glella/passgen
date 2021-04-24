//! # passclip
//!
//! Generates a random password and copies it to the clipboard
//!
//! -l optionally specifies length of password. Defaults to 16.
//!
//! -u optionally specifies number of uppercase characters. Defaults to 2.
//!
//! -d optionally specifies number of digits. Defaults to 2.
//!
//! -s optionally specifies number of special characters. Defaults to 2.

extern crate clipboard;
extern crate rand;
extern crate unicode_segmentation;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use rand::{seq::SliceRandom, Rng};
use unicode_segmentation::UnicodeSegmentation;

const LOWERSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITSET: &[u8] = b"0123456789";
const SPECIALSET: &[u8] = b")(*&^%$#@!~";

fn main() {
    let matches = App::new("passclip")
        .version("0.1.1")
        .author("Guillermo Lella <arkorott@gmail.com>")
        .about("Password Generator")
        .arg(
            Arg::with_name("len")
                .short("l")
                .value_name("length")
                .help("Password length, defaults to 16 when omitted"),
        )
        .arg(
            Arg::with_name("upp")
                .short("u")
                .value_name("upper")
                .help("Uppercase characters, defaults to 2 when omitted"),
        )
        .arg(
            Arg::with_name("dig")
                .short("d")
                .value_name("digits")
                .help("Number of digits, defaults to 2 when omitted"),
        )
        .arg(
            Arg::with_name("spe")
                .short("s")
                .value_name("special")
                .help("Special characters, defaults to 2 when omitted"),
        )
        .get_matches();

    let mut length: usize = value_t!(matches, "len", usize).unwrap_or(16);
    let digits: usize = value_t!(matches, "dig", usize).unwrap_or(2);
    let specials: usize = value_t!(matches, "spe", usize).unwrap_or(2);
    let uppers: usize = value_t!(matches, "upp", usize).unwrap_or(2);
    let lowers: usize;
    if length >= digits + specials + uppers {
        lowers = length - digits - specials - uppers;
    } else {
        length = digits + specials + uppers;
        lowers = length - digits - specials - uppers;
    }

    let password = gen_passw(lowers, uppers, digits, specials);

    println!("{}", password);

    // Copy password to clipboard
    let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
    clip.set_contents(password).unwrap();
}

fn gen_chars(len: usize, set: &[u8]) -> String {
    let mut rng = rand::thread_rng();
    let passchars: String = (0..len)
        .map(|_| {
            let index = rng.gen_range(0, set.len());
            set[index] as char
        })
        .collect();
    passchars
}

fn gen_passw(lowernum: usize, uppernum: usize, digitnum: usize, specialnum: usize) -> String {
    let mut s = String::new();

    let lowerchars = gen_chars(lowernum, LOWERSET);
    s.push_str(&lowerchars);

    let upperchars = gen_chars(uppernum, UPPERSET);
    s.push_str(&upperchars);

    let digichars = gen_chars(digitnum, DIGITSET);
    s.push_str(&digichars);

    let specialchars = gen_chars(specialnum, SPECIALSET);
    s.push_str(&specialchars);

    //println!("{}", s);
    let password: String = shuffle_str(&s);
    password
}

// Shuffles characters in a string (works with unicode)
fn shuffle_str(s: &str) -> String {
    let mut graphemes = s.graphemes(true).collect::<Vec<&str>>();
    let gslice = graphemes.as_mut_slice();
    let mut rng = rand::thread_rng();
    gslice.shuffle(&mut rng);
    gslice.iter().copied().collect::<String>()
}
