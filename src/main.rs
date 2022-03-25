use rust_0_2_prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
