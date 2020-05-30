fn main() {
    flexi_logger::Logger::with_env_or_str("info, mastocrom=trace")
        .format(flexi_logger::default_format)
        .start()
        .unwrap();
    mastocrom::mastodon::start().unwrap();
}
