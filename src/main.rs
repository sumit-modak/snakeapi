#[tokio::main]
async fn main() {
    let mut response = String::with_capacity(1024);

    response.push_str(&snakeapi::fetch_prompt());
    // response.push_str(&get_location().await);
    response.push_str(&snakeapi::fetch_os_info());
    response.push_str(&snakeapi::fetch_kernel_info());
    response.push_str(&snakeapi::fetch_shell_info());
    response.push_str(&snakeapi::fetch_uptime());
    response.push_str(&snakeapi::fetch_disk_usage());
    response.push_str(&snakeapi::fetch_memory());
    response.push_str(&snakeapi::fetch_cpu());
    response.push_str(&snakeapi::fetch_resolution());

    snakeapi::test::fs::check_path(".".to_string()).await;
    // snakeapi::ps::get_pstree().await;
    // snakeapi::ps::get_ps_list().await;

    println!("{response}");
}
