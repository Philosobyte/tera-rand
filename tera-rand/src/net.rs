use crate::common::{gen_value_in_range, parse_arg};
use crate::error::cidr_prefix_length_out_of_bounds;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use tera::{to_value, Result, Value};

/// A Tera function to generate a random IPv4 address.
///
/// The `start` parameter takes an IPv4 address to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `0.0.0.0`.
///
/// The `end` parameter also takes an IPv4 address indicating the end of the range,
/// which is also inclusive. An inclusive range allows `255.255.255.255` to be sampled where an
/// exclusive range does not. If `end` is not passed in, it defaults to `255.255.255.255`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_ipv4;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_ipv4", random_ipv4);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4(start="127.0.0.0", end="128.0.0.0") }}"#,  &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4(start="127.0.0.0") }}"#, &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4(end="128.0.0.0") }}"#, &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4() }}"#, &context)
///     .unwrap();
/// ```
pub fn random_ipv4(args: &HashMap<String, Value>) -> Result<Value> {
    let start_opt: Option<u32> = parse_arg(args, "start")?.map(|start: Ipv4Addr| start.into());

    let end_opt: Option<u32> = parse_arg(args, "end")?.map(|end: Ipv4Addr| end.into());

    let random_ipv4: u32 = gen_value_in_range(start_opt, end_opt, u32::MIN, u32::MAX);
    let random_ipv4: Ipv4Addr = random_ipv4.into();

    let json_value: Value = to_value(random_ipv4)?;
    Ok(json_value)
}

/// A Tera function to generate a random IPv6 address.
///
/// The `start` parameter takes an IPv6 address to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `::`.
///
/// The `end` parameter also takes an IPv6 address indicating the end of the range,
/// which is also inclusive. An inclusive range allows `ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff`
/// to be sampled where an exclusive range does not. If `end` is not passed in, it defaults to
/// `ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_ipv6;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_ipv6", random_ipv6);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6(start="fc00::", end="fd00::") }}"#, &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6(start="fc00::") }}"#, &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6(end="fd00::") }}"#, &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_ipv6() }}", &context)
///     .unwrap();
/// ```
pub fn random_ipv6(args: &HashMap<String, Value>) -> Result<Value> {
    let start_opt: Option<u128> =
        parse_arg(args, "start")?.map(|start_ipv6: Ipv6Addr| start_ipv6.into());

    let end_opt: Option<u128> = parse_arg(args, "end")?.map(|end_ipv6: Ipv6Addr| end_ipv6.into());

    let random_ipv6: u128 = gen_value_in_range(start_opt, end_opt, u128::MIN, u128::MAX);
    let random_ipv6: Ipv6Addr = random_ipv6.into();

    let json_value: Value = to_value(random_ipv6)?;
    Ok(json_value)
}

/// A Tera function to generate a random IPv4 CIDR address.
///
/// The `length_start` parameter takes an integer between 0 and 32 (inclusive) to indicate the
/// random prefix length of the generated CIDR should be at least `length_start`. If
/// `length_start` is not passed in, it defaults to 0.
///
/// The `length_end` parameter takes an integer between 0 and 32 (inclusive) to indicate the
/// random prefix length of the generated CIDR should be at most `length_end`. If
/// `length_end` is not passed in, it defaults to 32.
///
/// The `addr_start` parameter takes an IPv4 address. This address will be used as the inclusive
/// lower bound for generating the random address before the address is masked into a prefix.
/// If `addr_start` is not passed in, it defaults to `0.0.0.0`.
///
/// The `addr_end` parameter takes an IPv4 address. This address will be used as the inclusive
/// upper bound for generating the random address before the address is masked into a prefix.
/// If `addr_start` is not passed in, it defaults to `0.0.0.0`.
///
/// All of these parameters are optional, and it is possible to use any combination.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_ipv4_cidr;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_ipv4_cidr", random_ipv4_cidr);
/// let context: Context = Context::new();
///
/// // bound by neither prefix bit values nor the prefix length
/// let rendered: String = tera
///     .render_str("{{ random_ipv4_cidr() }}", &context)
///     .unwrap();
///
/// // prefix length bound by start and end lengths
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4_cidr(length_start=16, length_end=24) }}"#, &context)
///     .unwrap();
/// // prefix length bound by start length
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4_cidr(length_start=16) }}"#, &context)
///     .unwrap();
/// // prefix length bound by end length
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4_cidr(length_end=24) }}"#, &context)
///     .unwrap();
///
/// // prefix bits bound by a start address and end address
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4_cidr(addr_start="10.120.0.0", addr_end="10.140.0.0") }}"#, &context)
///     .unwrap();
/// // prefix bits bound by a start address
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4_cidr(start="10.120.0.0") }}"#, &context )
///     .unwrap();
/// // prefix bits bound by an end address
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv4_cidr(addr_end="10.140.0.0") }}"#, &context)
///     .unwrap();
///
/// // prefix length bound by both prefix bit values and length
/// let rendered: String = tera
///     .render_str(
///         r#"{{ random_ipv4_cidr(
///                   addr_start="10.120.0.0",
///                   addr_end="10.140.0.0",
///                   length_start=16,
///                   length_end=24
///               ) }}"#,
///         &context
///     )
///     .unwrap();
/// ```
pub fn random_ipv4_cidr(args: &HashMap<String, Value>) -> Result<Value> {
    let addr_start_opt: Option<u32> =
        parse_arg(args, "addr_start")?.map(|addr_start: Ipv4Addr| addr_start.into());
    let addr_end_opt: Option<u32> =
        parse_arg(args, "addr_end")?.map(|addr_end: Ipv4Addr| addr_end.into());

    let random_addr: u32 = gen_value_in_range(addr_start_opt, addr_end_opt, u32::MIN, u32::MAX);

    let length_start: u32 =
        parse_cidr_prefix_length_and_check_bounds(args, "length_start", 0u32, u32::BITS)?
            .unwrap_or(0u32);
    let length_end: u32 =
        parse_cidr_prefix_length_and_check_bounds(args, "length_end", 0u32, u32::BITS)?
            .unwrap_or(u32::BITS);

    let random_prefix_length: u32 = thread_rng().gen_range(length_start..=length_end);
    let bits_to_shift: u32 = u32::BITS - random_prefix_length;

    let random_prefix: u32 = match bits_to_shift {
        u32::BITS => 0u32,
        bits_to_shift => random_addr >> bits_to_shift << bits_to_shift,
    };
    let random_prefix: Ipv4Addr = random_prefix.into();

    let random_cidr: String = format!("{}/{}", random_prefix.to_string(), random_prefix_length);
    let json_value: Value = to_value(random_cidr)?;
    Ok(json_value)
}

/// A Tera function to generate a random IPv6 CIDR address.
///
/// The `length_start` parameter takes an integer between 0 and 128 (inclusive) to indicate the
/// random prefix length of the generated CIDR should be at least `length_start`. If
/// `length_start` is not passed in, it defaults to 0.
///
/// The `length_end` parameter takes an integer between 0 and 128 (inclusive) to indicate the
/// random prefix length of the generated CIDR should be at most `length_end`. If
/// `length_end` is not passed in, it defaults to 128.
///
/// The `addr_start` parameter takes an IPv4 address. This address will be used as the inclusive
/// lower bound for generating the random address before the address is masked into a prefix.
/// If `addr_start` is not passed in, it defaults to `::`.
///
/// The `addr_end` parameter takes an IPv4 address. This address will be used as the inclusive
/// upper bound for generating the random address before the address is masked into a prefix.
/// If `addr_start` is not passed in, it defaults to `ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff`.
///
/// All of these parameters are optional, and it is possible to use any combination.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_ipv6_cidr;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_ipv6_cidr", random_ipv6_cidr);
/// let context: Context = Context::new();
///
/// // bound by neither prefix bit values nor the prefix length
/// let rendered: String = tera
///     .render_str("{{ random_ipv6_cidr() }}", &context)
///     .unwrap();
///
/// // prefix length bound by start and end lengths
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6_cidr(length_start=64, length_end=80) }}"#, &context)
///     .unwrap();
/// // prefix length bound by start length
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6_cidr(length_start=64) }}"#, &context)
///     .unwrap();
/// // prefix length bound by end length
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6_cidr(length_end=80) }}"#, &context)
///     .unwrap();
///
/// // prefix bits bound by a start address and end address
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6_cidr(addr_start="fc00::", addr_end="fd00::") }}"#, &context)
///     .unwrap();
/// // prefix bits bound by a start address
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6_cidr(start="fc00::") }}"#, &context )
///     .unwrap();
/// // prefix bits bound by an end address
/// let rendered: String = tera
///     .render_str(r#"{{ random_ipv6_cidr(addr_end="fd00::") }}"#, &context)
///     .unwrap();
///
/// // prefix length bound by both prefix bit values and length
/// let rendered: String = tera
///     .render_str(
///         r#"{{ random_ipv6_cidr(
///                   addr_start="fc00::",
///                   addr_end="fd00::",
///                   length_start=64,
///                   length_end=80
///               ) }}"#,
///         &context
///     )
///     .unwrap();
/// ```
pub fn random_ipv6_cidr(args: &HashMap<String, Value>) -> Result<Value> {
    let addr_start_opt: Option<u128> =
        parse_arg(args, "addr_start")?.map(|addr_start: Ipv6Addr| addr_start.into());
    let addr_end_opt: Option<u128> =
        parse_arg(args, "addr_end")?.map(|addr_end: Ipv6Addr| addr_end.into());

    let random_addr: u128 = gen_value_in_range(addr_start_opt, addr_end_opt, u128::MIN, u128::MAX);

    let length_start: u32 =
        parse_cidr_prefix_length_and_check_bounds(args, "length_start", 0u32, u128::BITS)?
            .unwrap_or(0u32);
    let length_end: u32 =
        parse_cidr_prefix_length_and_check_bounds(args, "length_end", 0u32, u128::BITS)?
            .unwrap_or(u128::BITS);

    let random_prefix_length: u32 = thread_rng().gen_range(length_start..=length_end);
    let bits_to_shift: u32 = u128::BITS - random_prefix_length;

    let random_prefix: u128 = match bits_to_shift {
        u128::BITS => 0u128,
        bits_to_shift => random_addr >> bits_to_shift << bits_to_shift,
    };
    let random_prefix: Ipv6Addr = random_prefix.into();

    let random_cidr: String = format!("{}/{}", random_prefix.to_string(), random_prefix_length);
    let json_value: Value = to_value(random_cidr)?;
    Ok(json_value)
}

fn parse_cidr_prefix_length_and_check_bounds(
    args: &HashMap<String, Value>,
    parameter: &'static str,
    start_bound: u32,
    end_bound: u32,
) -> tera::Result<Option<u32>> {
    parse_arg(args, parameter)?
        .map(|length: u32| {
            if length < start_bound || length > end_bound {
                Err(cidr_prefix_length_out_of_bounds(
                    length,
                    start_bound,
                    end_bound,
                ))
            } else {
                Ok(length)
            }
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use crate::common::tests::{test_tera_rand_function, test_tera_rand_function_returns_error};
    use crate::net::*;
    use tracing_test::traced_test;

    // ipv4 address
    #[test]
    #[traced_test]
    fn test_random_ipv4() {
        test_tera_rand_function(
            random_ipv4,
            "random_ipv4",
            r#"{ "some_field": "{{ random_ipv4() }}" }"#,
            r#"\{ "some_field": "\d+\.\d+\.\d+\.\d+" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_with_both_start_and_end() {
        test_tera_rand_function(
            random_ipv4,
            "random_ipv4",
            r#"{ "some_field": "{{ random_ipv4(start="127.0.0.1", end="127.0.0.3") }}" }"#,
            r#"\{ "some_field": "(127\.0\.0\.1|127\.0\.0\.2|127\.0\.0\.3)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_near_max() {
        test_tera_rand_function(
            random_ipv4,
            "random_ipv4",
            r#"{ "some_field": "{{ random_ipv4(start="255.255.255.253", end="255.255.255.255") }}" }"#,
            r#"\{ "some_field": "(255\.255\.255\.253|255\.255\.255\.254|255\.255\.255\.255)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_near_min() {
        test_tera_rand_function(
            random_ipv4,
            "random_ipv4",
            r#"{ "some_field": "{{ random_ipv4(start="0.0.0.0", end="0.0.0.2") }}" }"#,
            r#"\{ "some_field": "(0\.0\.0\.0|0\.0\.0\.1|0\.0\.0\.2)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_with_start_only() {
        test_tera_rand_function(
            random_ipv4,
            "random_ipv4",
            r#"{ "some_field": "{{ random_ipv4(start="255.255.255.253") }}" }"#,
            r#"\{ "some_field": "(255\.255\.255\.253|255\.255\.255\.254|255\.255\.255\.255)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_with_end_only() {
        test_tera_rand_function(
            random_ipv4,
            "random_ipv4",
            r#"{ "some_field": "{{ random_ipv4(end="0.0.0.2") }}" }"#,
            r#"\{ "some_field": "(0\.0\.0\.0|0\.0\.0\.1|0\.0\.0\.2)" }"#,
        );
    }

    // ipv6 address
    #[test]
    #[traced_test]
    fn test_random_ipv6() {
        test_tera_rand_function(
            random_ipv6,
            "random_ipv6",
            r#"{ "some_field": "{{ random_ipv6() }}" }"#,
            r#"\{ "some_field": "([\da-f]{0,4}:){1,7}[\da-f]{0,4}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_with_both_start_and_end() {
        test_tera_rand_function(
            random_ipv6,
            "random_ipv6",
            r#"{ "some_field": "{{ random_ipv6(start="fe80::", end="fe80::2") }}" }"#,
            r#"\{ "some_field": "(fe80::|fe80::1|fe80::2)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_near_max() {
        test_tera_rand_function(
            random_ipv6,
            "random_ipv6",
            r#"{ "some_field": "{{ random_ipv6(start="ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffd", end="ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff") }}" }"#,
            r#"\{ "some_field": "ffff:ffff:ffff:ffff:ffff:ffff:ffff:fff(d|e|f)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_near_min() {
        test_tera_rand_function(
            random_ipv6,
            "random_ipv6",
            r#"{ "some_field": "{{ random_ipv6(start="::", end="::2") }}" }"#,
            r#"\{ "some_field": "(::|::1|::2)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_with_start_only() {
        test_tera_rand_function(
            random_ipv6,
            "random_ipv6",
            r#"{ "some_field": "{{ random_ipv6(start="ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffd") }}" }"#,
            r#"\{ "some_field": "ffff:ffff:ffff:ffff:ffff:ffff:ffff:fff(d|e|f)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_with_end_only() {
        test_tera_rand_function(
            random_ipv6,
            "random_ipv6",
            r#"{ "some_field": "{{ random_ipv6(end="::2") }}" }"#,
            r#"\{ "some_field": "(::|::1|::2)" }"#,
        );
    }

    // ipv4 cidr
    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr() {
        test_tera_rand_function(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr() }}" }"#,
            r#"\{ "some_field": "\d+\.\d+\.\d+\.\d+/\d+" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_prefix_length_start_and_end() {
        test_tera_rand_function(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_start=28, length_end=30) }}" }"#,
            r#"\{ "some_field": "\d+\.\d+\.\d+\.\d+/(28|29|30)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_prefix_length_start() {
        test_tera_rand_function(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_start=30) }}" }"#,
            r#"\{ "some_field": "\d+\.\d+\.\d+\.\d+/(30|31|32)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_prefix_length_end() {
        test_tera_rand_function(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_end=2) }}" }"#,
            r#"\{ "some_field": "\d+\.\d+\.\d+\.\d+/(0|1|2)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_32_bit_prefix() {
        test_tera_rand_function(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_start=32, length_end=32) }}" }"#,
            r#"\{ "some_field": "\d+\.\d+\.\d+\.\d+/32" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_0_bit_prefix() {
        test_tera_rand_function(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_start=0, length_end=0) }}" }"#,
            r#"\{ "some_field": "0\.0\.0\.0/0" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_too_large_prefix_length_returns_error() {
        test_tera_rand_function_returns_error(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_start=0, length_end=33) }}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv4_cidr_with_too_small_prefix_length_returns_error() {
        test_tera_rand_function_returns_error(
            random_ipv4_cidr,
            "random_ipv4_cidr",
            r#"{ "some_field": "{{ random_ipv4_cidr(length_start=-1, length_end=16) }}" }"#,
        );
    }

    // ipv6 cidr
    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr() {
        test_tera_rand_function(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr() }}" }"#,
            r#"\{ "some_field": "([\da-f]{0,4}:){1,7}[\da-f]{0,4}/\d+" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_prefix_length_start_and_end() {
        test_tera_rand_function(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_start=86, length_end=88) }}" }"#,
            r#"\{ "some_field": "([\da-f]{0,4}:){1,7}[\da-f]{0,4}/(86|87|88)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_prefix_length_start() {
        test_tera_rand_function(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_start=126) }}" }"#,
            r#"\{ "some_field": "([\da-f]{0,4}:){1,7}[\da-f]{0,4}/(126|127|128)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_prefix_length_end() {
        test_tera_rand_function(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_end=2) }}" }"#,
            r#"\{ "some_field": "([\da-f]{0,4}:){1,7}[\da-f]{0,4}/(0|1|2)" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_128_bit_prefix() {
        test_tera_rand_function(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_start=128, length_end=128) }}" }"#,
            r#"\{ "some_field": "([\da-f]{0,4}:){1,7}[\da-f]{0,4}/128" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_0_bit_prefix() {
        test_tera_rand_function(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_start=0, length_end=0) }}" }"#,
            r#"\{ "some_field": "::/0" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_too_large_prefix_length_returns_error() {
        test_tera_rand_function_returns_error(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_start=0, length_end=129) }}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_ipv6_cidr_with_too_small_prefix_length_returns_error() {
        test_tera_rand_function_returns_error(
            random_ipv6_cidr,
            "random_ipv6_cidr",
            r#"{ "some_field": "{{ random_ipv6_cidr(length_start=-1, length_end=16) }}" }"#,
        );
    }
}
