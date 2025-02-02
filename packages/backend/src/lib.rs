uniffi::setup_scaffolding!();

pub mod backend {
    #[uniffi::export]
    pub fn get_version() -> String {
        String::from(env!("CARGO_PKG_VERSION"))
    }
}
