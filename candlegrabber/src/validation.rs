pub fn validate_duration_str(input: &str) -> bool {
    input == "1s" ||
        input == "5s" ||
        input == "15s" ||
        input == "30s" ||
        input == "1m" ||
        input == "5m" ||
        input == "15m" ||
        input == "30m" ||
        input == "1h" ||
        input == "2h" ||
        input == "4h" ||
        input == "6h" ||
        input == "8h" ||
        input == "12h" ||
        input == "1d" ||
        input == "3d" ||
        input == "1w" ||
        input == "1M"
}