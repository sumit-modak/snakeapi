use std::process::Stdio;
use tokio::process::Command;

pub async fn test_this_fn() {
    println!("PID: {}", std::process::id());
    test_piping().await;
    hello().await;
}

pub async fn hello() {
    let mut sleep = Command::new("sleep")
        .arg("60")
        // .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn sleep");

    println!("{:?}", sleep.id());
    println!("{:?}", sleep.wait().await);
}

// fd[0] = read end
// fd[1] = write end
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

    let (echo_result, tr_output) = tokio::join!(echo.wait(), tr.wait_with_output());

    println!("{:?}", echo_result.unwrap().success());

    let tr_output = tr_output.expect("failed to await tr");
    println!("{:?}", tr_output.status.success());

    println!("{:?}", String::from_utf8(tr_output.stdout).unwrap());
}
