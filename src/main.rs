use ansi_term::Colour::*;
use std::collections::HashSet;
use std::process;

/// Make boring stuff dull
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args2: Vec<String> = std::env::args().skip(2).collect();

    println!("a");
    let out = process::Command::new(args[0].clone())
        .args(args2)
        .output()
        .unwrap();

    println!("{}", bore(String::from_utf8(out.stdout).unwrap()));
    println!("{}", bore(String::from_utf8(out.stderr).unwrap()));
}

//TODO streams!
fn bore(inst: String) -> String {
    let mut res = String::new();

    //TODO regex
    let warning = format!("{}", Red.paint("warning"));
    let error = format!("{}", Red.paint("error"));
    let cap_error = format!("{}", Red.paint("Error"));
    let cap_cap_error = format!("{}", Red.paint("ERROR"));
    let exception = format!("{}", Red.paint("exception"));

    //todo trie
    let mut before: HashSet<String> = HashSet::new();

    for line in inst.lines() {
        let mut l: String = line.to_string();

        if before.contains(line) {
            //If the whole lines been seen before it's boring.
            l = format!("{}", RGB(128, 128, 128).paint(&l));
            for part in line.split(' ') {
                before.insert(part.to_string());
            }
        } else {
            let mut buf = String::new();
            for part in l.split(' ') {
                if before.contains(part) {
                    buf.push_str(&format!("{}", RGB(128, 128, 128).paint(part)))
                } else {
                    buf.push_str(part.clone());
                }
                buf.push(' ');

                before.insert(part.to_string());
            }
            l = buf;
        }

        before.insert(line.to_string());

        l = l.replace("warning", &warning);
        l = l.replace("error", &error);
        l = l.replace("Error", &cap_error);
        l = l.replace("ERROR", &cap_cap_error);
        l = l.replace("exception", &exception);
        res.push_str(&l);
        res.push('\n');
    }
    res
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn intesting() {}
}