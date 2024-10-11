// tests/config_args_tests.rs
mod config_args_tests {
    use port_scanner::config::args::{parse_ports, validate_ip_or_hostname};

    #[test]
    fn test_valid_ip_addresses() {
        assert!(validate_ip_or_hostname("192.168.1.1").is_ok());
        assert!(validate_ip_or_hostname("::1").is_ok()); // IPv6
    }

    #[test]
    fn test_invalid_ip_addresses() {
        let res1 = validate_ip_or_hostname("999.999.999.999");
        println!("Result for 999.999.999.999: {:?}", res1);
        assert!(res1.is_err());

        let res2 = validate_ip_or_hostname("abcd::12345");
        println!("Result for abcde::12345: {:?}", res2);
        assert!(res2.is_err());
    }

    #[test]
    fn test_valid_hostnames() {
        assert!(validate_ip_or_hostname("example.com").is_ok());
        assert!(validate_ip_or_hostname("localhost").is_ok());
    }

    #[test]
    fn test_invalid_hostnames() {
        assert!(validate_ip_or_hostname("example..com").is_err());
        assert!(validate_ip_or_hostname("-example.com").is_err());
    }

    #[test]
    fn test_single_ports() {
        let result = parse_ports("80").unwrap();
        assert_eq!(result, vec![80]);
    }

    #[test]
    fn test_port_range() {
        let result = parse_ports("20-22").unwrap();
        assert_eq!(result, vec![20, 21, 22]);
    }

    #[test]
    fn test_mixed_ports_and_ranges() {
        let result = parse_ports("22, 80, 443-445").unwrap();
        assert_eq!(result, vec![22, 80, 443, 444, 445]);
    }

    #[test]
    fn test_invalid_ports() {
        assert!(parse_ports("99999").is_err());
        assert!(parse_ports("80-22").is_err());
        assert!(parse_ports("abc").is_err());
    }
}
