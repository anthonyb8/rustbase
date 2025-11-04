// tests/common.rs
use once_cell::sync::Lazy;

static INIT: Lazy<()> = Lazy::new(|| {
    let _ = dotenvy::dotenv();
    println!("✅ .env loaded for tests");
});

pub fn setup() {
    Lazy::force(&INIT);
}
