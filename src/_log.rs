pub fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[cfg(test)]
mod test_log {

    #[test]
    fn test_log() {
        crate::_log::init();

        info!("log init ok");
    }
}
