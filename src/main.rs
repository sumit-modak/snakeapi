use gentle_snake::*;

#[tokio::main]
async fn main() {
    // ps::get_pstree().await;
    // ps::get_ps_list().await;
    fs::check_path(".".to_string()).await;

    // test::test_this_fn().await;
}
