// STARK-witness, a tool for parsing memory debug logs

// SPDX-FileCopyrightText: © 2023 Matthew Rothlisberger
// SPDX-License-Identifier: GPL-3.0-only

// <>

// src/main.rs

// Process a file containing output from the STARK program when
// compiled with the "memdbg" feature; produce useful summaries

// <>

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Provide a file containing STARK memdbg readout.");
        std::process::exit(1);
    }
    let target_readout = &args[1];

    let mut objs = std::collections::HashMap::new();

    let file_contents = std::fs::read_to_string(target_readout).unwrap();

    let mut highest_id = 0;

    for line in file_contents.lines() {
        let mut c = line.bytes();

        match c.next() {
            Some(b'O') => (),
            Some(_) => continue,
            None => continue,
        }

        if c.next() != Some(b' ') {
            continue;
        }

        let mut acc = 0;
        for d in c.by_ref() {
            if d == b' ' {
                break;
            }
            acc *= 10;
            acc += d as u32 - 48;
        }
        let code_start = c.next().unwrap();
        if code_start == b'B' && c.next().unwrap() == b'I' && c.next().unwrap() == b'R' {
            let mut typ = String::new();
            while c.next().unwrap() != b't' {}
            c.next();
            for ch in c.by_ref() {
                if ch == b')' {
                    break;
                }
                typ.push(char::from(ch))
            }

            if objs.insert(acc, typ).is_some() {
                panic!()
            }

            if acc >= highest_id {
                highest_id = acc
            } else {
                panic!()
            }
        } else if code_start == b'R' && c.next().unwrap() == b'E' && c.next().unwrap() == b'C' {
            // TODO: detect use after free & double free
            if objs.remove(&acc).is_none() {
                panic!("object {acc} not found")
            }
        }
    }

    println!("{:?}", objs);
    println!("Total objects created: {}", highest_id + 1);
    println!("Total objects not reclaimed: {}", objs.len());
}
