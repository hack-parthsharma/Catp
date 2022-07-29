use catp::{catp, CatpArgs};
use std::{io::Read, process, thread};

#[test]
fn hello() {
    let mut p = process::Command::new("tests/scripts/hello")
        .stdin(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .unwrap();

    let args = CatpArgs {
        pid: p.id(),
        verbose: false,
    };

    let t = thread::spawn(move || {
        let mut stdout: Vec<u8> = vec![];
        let mut stderr: Vec<u8> = vec![];
        catp(args, &mut stdout, &mut stderr).unwrap();
        (stdout, stderr)
    });

    // Start hello
    drop(p.stdin.take());

    let (actual_stdout, actual_stderr) = t.join().unwrap();
    let _ = p.wait();

    let mut expected_stderr = vec![];
    p.stderr
        .take()
        .unwrap()
        .read_to_end(&mut expected_stderr)
        .unwrap();

    let mut expected_stdout = vec![];
    p.stdout
        .take()
        .unwrap()
        .read_to_end(&mut expected_stdout)
        .unwrap();

    assert_eq!(actual_stdout, expected_stdout);
    assert_eq!(actual_stderr, expected_stderr);
}
