use std::process::Stdio;
use tokio::join;
use tokio::process::Command;

pub async fn test_this_fn() {
    test_piping().await;
}

pub async fn test_piping() {
    let mut echo = Command::new("echo")
        .arg("hello world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn echo");

    let tr_stdin: Stdio = echo
        .stdout
        .take()
        .unwrap()
        .try_into()
        .expect("failed to convert to Stdio");

    let tr = Command::new("tr")
        .args(["a-z", "A-Z"])
        .stdin(tr_stdin)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn tr");

    let (echo_result, tr_output) = join!(echo.wait(), tr.wait_with_output());

    println!("{:?}", echo_result.unwrap().success());

    let tr_output = tr_output.expect("failed to await tr");
    println!("{:?}", tr_output.status.success());

    println!("{:?}", String::from_utf8(tr_output.stdout).unwrap());
}
