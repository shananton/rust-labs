#![feature(array_windows)]

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, Match};
use crate::encoding_part::EncodingPart;
use crate::mapper::WordToNumberMapper;

mod utils;
mod mapper;
mod encoding_part;

// 4 MiB
const BUF_CAPACITY: usize = 4 * (1 << 20);
const MAX_NUMBER_LENGTH: usize = 50;

fn usage_error(prog_name: &str) {
    eprintln!("Usage: {} <dict_file> <nums_file>", prog_name);
    std::process::exit(-1);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        usage_error(args.get(0).map(|s| s.as_str()).unwrap_or("phone_numbers"));
    }

    let mapper = WordToNumberMapper::new(&[
        b"e",
        b"jnq",
        b"rwx",
        b"dsy",
        b"ft",
        b"am",
        b"civ",
        b"bku",
        b"lop",
        b"ghz"
    ], b"-\"");

    let mut words_buf = Vec::new();
    let mut words_indices = Vec::new();
    let mut assoc_numbers_buf = Vec::new();
    let mut assoc_numbers_indices = Vec::new();

    for_each_line(must_open_file(&args[1]), |word| {
        words_indices.push(words_buf.len());
        words_buf.extend_from_slice(word);
        assoc_numbers_indices.push(assoc_numbers_buf.len());
        assoc_numbers_buf.extend(mapper.word_to_number(word));
    });
    words_indices.push(words_buf.len());
    assoc_numbers_indices.push(assoc_numbers_buf.len());

    let matcher: AhoCorasick<u32> = {
        let patterns = assoc_numbers_indices.array_windows::<2>()
            .map(|&[begin, end]| &assoc_numbers_buf[begin..end]);
        AhoCorasickBuilder::new()
            .dfa(true)
            .build_with_size(patterns).expect("Failed to build Aho-Corasick automaton")
    };

    let mut writer = BufWriter::with_capacity(BUF_CAPACITY, std::io::stdout());
    type Matches = [Vec<Match>; MAX_NUMBER_LENGTH];
    let mut matches: Matches = make_shared_match_buf();
    let mut filtered_num_buf = [0u8; MAX_NUMBER_LENGTH];
    for_each_line(must_open_file(&args[2]), |number| {
        let mut len = 0;
        for &digit in number.iter().filter(|&c| c.is_ascii_digit()) {
            filtered_num_buf[len] = digit;
            len += 1;
        }
        for i in 0..len {
            matches[i].clear();
        }
        let filtered_num = &filtered_num_buf[..len];
        for m in matcher.find_overlapping_iter(filtered_num) {
            matches[m.start()].push(m);
        }

        fn write_all_encodings(pos: usize,
                               part: Option<&EncodingPart>,
                               can_skip: bool,
                               orig_num: &[u8],
                               filtered_num: &[u8],
                               writer: &mut impl Write,
                               matches: &Matches,
                               words_buf: &[u8],
                               words_indices: &[usize],
        ) -> std::io::Result<()> {
            if pos == filtered_num.len() {
                writer.write_all(orig_num)?;
                writer.write_all(b": ")?;
                match part {
                    Some(part) => { part.write_into(writer)?; }
                    None => ()
                }
                writer.write_all(b"\n")?;
                return Ok(());
            }

            for m in &matches[pos] {
                let i = m.pattern();
                let part = EncodingPart {
                    word: &words_buf[words_indices[i]..words_indices[i + 1]],
                    prev: part,
                };
                write_all_encodings(m.end(), Some(&part), true,
                                    orig_num, filtered_num, writer, matches, words_buf, words_indices)?;
            }

            if matches[pos].is_empty() && can_skip {
                let part = EncodingPart {
                    word: &filtered_num[pos..=pos],
                    prev: part,
                };
                write_all_encodings(pos + 1, Some(&part), false,
                                    orig_num, filtered_num, writer, matches, words_buf, words_indices)?;
            }

            Ok(())
        }

        write_all_encodings(0, None, true,
                            number, filtered_num, &mut writer, &matches, &words_buf, &words_indices)
            .expect("write failed");
    });
}

fn make_shared_match_buf() -> [Vec<Match>; MAX_NUMBER_LENGTH] {
    unsafe {
        let mut buf: [std::mem::MaybeUninit<Vec<usize>>; MAX_NUMBER_LENGTH] =
            std::mem::MaybeUninit::uninit().assume_init();
        for v in &mut buf {
            v.write(Vec::new());
        }
        std::mem::transmute_copy(&buf)
    }
}

fn map_lines<R, F: FnMut(&[u8]) -> R>(file: File, mut func: F) -> impl Iterator<Item=R> {
    let mut reader = BufReader::with_capacity(BUF_CAPACITY, file);
    let mut buf = Vec::new();
    std::iter::from_fn(move || {
        let result = match reader.read_until(b'\n', &mut buf) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(func(buf.strip_suffix(b"\n").unwrap_or(&buf)))
        };
        buf.clear();
        result
    })
}

fn for_each_line<F: FnMut(&[u8])>(file: File, func: F) {
    for _ in map_lines(file, func) {}
}

fn must_open_file(path: &str) -> File {
    File::open(path).expect(&format!("Failed to open file: {}", path))
}