use gentle_snake::{ps, test};

#[tokio::main]
async fn main() {
    // ps::get_pstree().await;
    // ps::get_ps_list().await;
    test::test_this_fn().await;
}
