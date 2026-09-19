//! Black-box CLI checks. No private constructors, test hooks or operational custody.
use std::io::Write;
use std::process::{Command, Stdio};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_dgr-local-issuer")
}
fn invoke(args: &[&str], input: &[u8]) -> std::process::Output {
    let mut child = Command::new(binary())
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Rejection may close stdin before the parent finishes its public invalid input.
    let mut stdin = child.stdin.take().unwrap();
    let _ = stdin.write_all(input);
    drop(stdin);
    child.wait_with_output().unwrap()
}
#[test]
fn closed_commands_and_constant_public_help() {
    let version = invoke(&["--version"], b"");
    assert!(version.status.success());
    assert_eq!(version.stdout, b"dgr-local-issuer 0.1.0\n");
    assert!(version.stderr.is_empty());
    let help = invoke(&["--help"], b"");
    assert!(help.status.success());
    assert!(
        help.stdout
            .starts_with(b"Usage: dgr-local-issuer --help | --version | issue\n")
    );
    assert!(help.stderr.is_empty());
    for args in [
        vec![],
        vec!["help"],
        vec!["--help", "issue"],
        vec!["issue", "--nonce", "PUBLIC"],
        vec!["issue", "--key", "PUBLIC"],
        vec!["issue", "--test"],
        vec!["issue", "--profile", "PUBLIC"],
        vec!["issue", "--time", "0"],
        vec!["--version", "--help"],
    ] {
        let output = invoke(&args, b"PUBLIC INVALID INPUT");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(output.stderr, b"DGR-E002 invocation\n");
    }
}
#[test]
fn request_rejections_are_fixed_and_redacted() {
    for input in [
        b"".as_slice(),
        b"{}\n",
        b"PUBLIC-CALLER-TEXT-MUST-NOT-APPEAR",
        &[b'X'; 246],
    ] {
        let output = invoke(&["issue"], input);
        assert_eq!(output.status.code(), Some(3));
        assert!(output.stdout.is_empty());
        assert_eq!(output.stderr, b"DGR-E003 request\n");
    }
    let output = Command::new(binary())
        .arg("issue")
        .env("DGR_ISSUER_TEST", "1")
        .env("DGR_SIGNING_KEY", "PUBLIC-NOT-A-KEY")
        .env("RUST_BACKTRACE", "full")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"DGR-E003 request\n");
}
#[test]
fn socket_stdin_is_rejected() {
    let (read, _write) = std::os::unix::net::UnixStream::pair().unwrap();
    let fd: std::os::fd::OwnedFd = read.into();
    let output = Command::new(binary())
        .arg("issue")
        .stdin(Stdio::from(fd))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"DGR-E003 request\n");
}
#[test]
fn pipe_requires_eof_before_absolute_deadline() {
    let start = std::time::Instant::now();
    let mut child = Command::new(binary())
        .arg("issue")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"{").unwrap();
    let output = child.wait_with_output().unwrap();
    drop(stdin);
    assert!(start.elapsed() >= std::time::Duration::from_secs(9));
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"DGR-E003 request\n");
}

#[test]
fn insufficient_inherited_address_space_is_a_fixed_resource_failure() {
    use std::os::unix::process::CommandExt;
    for limit in [128 * 1024 * 1024, 256 * 1024 * 1024] {
        let mut command = Command::new(binary());
        command.arg("issue").stdin(Stdio::null());
        // SAFETY: this callback runs only in the forked child before exec and calls
        // only setrlimit with a stack value; it does not allocate or acquire locks.
        unsafe {
            command.pre_exec(move || {
                let limits = libc::rlimit {
                    rlim_cur: limit,
                    rlim_max: limit,
                };
                if libc::setrlimit(libc::RLIMIT_AS, &limits) == 0 {
                    Ok(())
                } else {
                    Err(std::io::Error::last_os_error())
                }
            });
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(13));
        assert!(output.stdout.is_empty());
        assert_eq!(output.stderr, b"DGR-E013 resource\n");
    }
}
