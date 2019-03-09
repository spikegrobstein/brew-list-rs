use std::process::{Command, Stdio};
use std::thread;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;

fn main() {
    let mut cmd = Command::new("brew");

    cmd
        .arg("list")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    let mut result = cmd.spawn().expect("Failed to execute");

    eprintln!("Starting brew process with id {}", result.id());

    let stdout = result.stdout.take().expect("could not get stdout");
    let stderr = result.stderr.take().expect("failed to get stderr");

    let (tx, rx) = mpsc::channel();

    let out_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(line) => tx.send(line).unwrap(), //println!("[OUT] {}", line),
                Err(error) => panic!("ERROR [OUT]: {}", error),
            }
        }
    });

    let err_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            match line {
                Ok(line) => println!("[ERR] {}", line),
                Err(error) => panic!("ERROR [ERR]: {}", error),
            }
        }
    });

    let mut packages: Vec<String> = vec![];

    for pkg in rx {
        packages.push(pkg);
    }

    let status = result.wait().expect("Uh oh");

    out_thread.join().expect("failed to join stdout thread");
    err_thread.join().expect("failed to join stderr thread");

    println!("Done: {}", status.code().expect("Got no code"));

    println!("Got {} packages:", packages.len());

    for pkg in packages.iter() {
        println!(" - {}", pkg);
    }
}
