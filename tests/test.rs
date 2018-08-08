#[cfg(test)]
extern crate game_kernel;
mod tests {
    use super::game_kernel::game_kernel::logger;
    #[test]
    fn logger_test() {
        logger::log_msg("this is a message");
        logger::log_warn("this is a message");
        logger::log_err("this is a message");
    }
}
