use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();
}
