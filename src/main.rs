use std::io::{stdin, stdout, Write, Read};
use std::str;
use std::io;

fn insert_chars(m: Vec<&str>, pos: i32) -> Vec<&str> {
    let buf = &[0xe2, 0x80, 0xab];
    let buf_str = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let mut new_vec = Vec::new();
    for (_, &s) in m[..pos as usize].iter().enumerate() {
        new_vec.push(s)
    }
    new_vec.push(buf_str);
    for (_, &s) in m[pos as usize..].iter().enumerate() {
        new_vec.push(s)
    }
    new_vec.push(buf_str);
    new_vec
}

fn main() {
    print!("========================\nDiscord (edited) Position Changer\nhttps://github.com/vysiondev/discord-edited-changer\n========================\n");
    let mut s = String::new();
    print!("Enter text that you want to insert (edited) into.\n>> ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Something went wrong when parsing the string.");
    let str_vec = s.trim().split(" ").collect::<Vec<&str>>();

    let index: i32;
    print!("Enter the index of where the (edited) text should be inserted into.\nThe index is zero-based. This means that if you choose \"0\" it will insert the (edited) in the very beginning, and so on.\n");
    loop {
        print!("\nIndex # will be >> ");
        let _ = stdout().flush();
        let mut index_str = String::new();
        stdin().read_line(&mut index_str).expect("Something went wrong when parsing the index to use.");
        println!("{}", &index_str);
        let index_proposed = match index_str.trim().parse::<i32>() {
            Ok(t) => t,
            Err(w) => {
                println!("{:?}", w);
                continue;
            }
        };

        if index_proposed < 0 {
            print!("Index should be greater than 0.\n");
            continue
        }
        if index_proposed > str_vec.len() as i32 {
            print!("Index should not exceed the string length.\n");
            continue
        }
        let bef = if index_proposed - 1 < 0 { "" } else { &str_vec[index_proposed as usize - 1] };
        let aft = if index_proposed >= str_vec.len() as i32 { "" } else{ &str_vec[index_proposed as usize] };
        print!("The (edited) text will be inserted between \"{}\" and \"{}\". Continue? [y/n]\n", bef, aft);

        let mut decision = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut decision).expect("Could not parse your answer.");
        if decision.trim().to_lowercase().starts_with("y") {
            index = index_proposed;
            break
        }
    }

    print!("Finished. Edit your message with this new content:\n{}\n\n", insert_chars(str_vec, index).join(" "));

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press ENTER to close this window...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}