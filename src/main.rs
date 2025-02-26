use thread_safe_rust::infrastructure::cli::run;

#[tokio::main]
async fn main() {
    run().await;
}
