//use async_std::io::{self, Read};
//use async_std::task;
use std::error::Error;
use ansi_term::Colour;
//use subprocess::*;
use std::collections::HashSet;

fn main() {
    //rintln!("ok {}", buffer);

    let args:Vec<String> = std::env::args().skip(1).collect();
    let args2:Vec<String> = std::env::args().skip(2).collect();

    // let mut p = Popen::create(&args, PopenConfig {
    //     stderr: Redirection::Merge, ..Default::default()
    // }).unwrap();
    println!("a");
//    let x : String = p.stream_stdin().unwrap();
    // Obtain the output from the standard streams.
//    let (out, err) = p.communicate(None).unwrap();
    let out = std::process::Command::new(args[0].clone()).args(args2).output().unwrap();

//    p.communicate_bites(None)

        println!("{}", fo(String::from_utf8(out.stdout).unwrap()));
        println!("{}", fo(String::from_utf8(out.stderr).unwrap()));


 //   let stderr = io::stderr();
  //  let mut buf = String::new();
  //  let mut handle = stderr.read_line(&mut buf).await;

    //Ok(())
}


fn fo (inst :String) -> String {
    let mut res = String::new();

    use ansi_term::Colour::*;
//Colour::RGB(255,165,10)
    let warning = format!("{}", Red.paint("warning"));
    let error = format!("{}", Red.paint("error"));
    let cap_error = format!("{}", Red.paint("Error"));
    let cap_cap_error = format!("{}", Red.paint("ERROR"));
    let exception = format!("{}", Red.paint("exception"));

    let mut prev_line : String = "".into();

    let mut before : HashSet<String> = HashSet::new();

    for line in inst.lines() {
        let mut l: String = line.to_string();

        let (from, to) = find_interesting(&prev_line, &l);

        if before.contains(line) {
            //If the whole lines been seen before it's boring.
            l = format!("{}",RGB(128,128,128).paint(&l));
            for part in line.split(' ') {
                before.insert(part.to_string());
            }
        } else {
//            if from > 0 {
//                l = format!("{}",RGB(128,128,128).paint(&l[0..from])) + &l[from..to] + &l[to..l.len()];
//            }
            let mut buf = String::new();
            for part in l.split(' ') {
                if before.contains(part) {
                    buf.push_str(&format!("{}",RGB(128,128,128).paint(part)))
                }
                else {
                    buf.push_str(part.clone());
                }
                buf.push(' ');

                before.insert(part.to_string());
            }
            l = buf;
        }

        before.insert(line.to_string());


        if l.contains("warning") {
            l = line.replace("warning", &warning);
        }
        if l.contains("error") {
            l =line.replace("error", &error);
        }
        if l.contains("Error") {
            l =line.replace("Error", &cap_error);
        }
        if l.contains("ERROR") {
            l =line.replace("ERROR", &cap_cap_error);
        }
        if l.contains("exception") {
            l =line.replace("exception", &exception);
        }
        res.push_str(&l);
        prev_line = line.to_string();
        res.push('\n');
    }
    res
}


/// Anything repeated is boring.
fn find_interesting(first: &str, second: &str) -> (usize, usize) {
    //from start
    let mut start = 0;
    let fi :Vec<_> = first.chars().collect();
    for (i, ch) in second.chars().enumerate()
    {
        if i >= fi.len() || ch != fi[i] {
            start = if i == 0 { 0 } else { i - 1};
            break;
        }
    }

    (start, second.len())

}

#[cfg(test)]
pub mod tests {
    #[test]
    fn intesting() {

    }
}