use crate::common::parse_range_and_gen_value_in_range;
use rand::random;
use std::collections::HashMap;
use tera::{to_value, Result, Value};

/// A Tera function to generate a random boolean.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_bool;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_bool", random_bool);
///
/// let context: Context = Context::new();
/// let rendered: String = tera.render_str("{{ random_bool() }}", &context).unwrap();
/// ```
pub fn random_bool(_args: &HashMap<String, Value>) -> tera::Result<Value> {
    let random_value: bool = random::<bool>();
    let json_value: Value = to_value(random_value)?;
    Ok(json_value)
}

/// A Tera function to generate a random char.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_char;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_char", random_char);
///
/// let context: Context = Context::new();
/// let rendered: String = tera.render_str("{{ random_char() }}", &context).unwrap();
/// ```
pub fn random_char(_args: &HashMap<String, Value>) -> Result<Value> {
    let random_value: char = random::<char>();
    let json_value: Value = to_value(random_value)?;
    Ok(json_value)
}

/// A Tera function to generate a random unsigned 32-bit integer.
///
/// The `start` parameter takes an unsigned 32-bit integer to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `u32::MIN`.
///
/// The `end` parameter also takes an unsigned 32-bit integer indicating the end of the range,
/// which is also inclusive. An inclusive range allows `u32::MAX` to be sampled where an
/// exclusive range does not. If `end` is not passed in, it defaults to `u32::MAX`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither in order to
/// sample across the entire `u32` space.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_uint32;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_uint32", random_uint32);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str("{{ random_uint32(start=49152, end=65535) }}", &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str("{{ random_uint32(start=4294967290) }}", &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str("{{ random_uint32(end=65535) }}", &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_uint32() }}", &context)
///     .unwrap();
/// ```
pub fn random_uint32(args: &HashMap<String, Value>) -> Result<Value> {
    parse_range_and_gen_value_in_range(args, u32::MIN, u32::MAX)
}

/// A Tera function to generate a random unsigned 64-bit integer.
///
/// The `start` parameter takes an unsigned 64-bit integer to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `u64::MIN`.
///
/// The `end` parameter also takes an unsigned 64-bit integer indicating the end of the range,
/// which is also inclusive. An inclusive range allows `u64::MAX` to be sampled where an
/// exclusive range does not. If `end` is not passed in, it defaults to `u64::MAX`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither in order to
/// sample across the entire `u64` space.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_uint64;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_uint64", random_uint64);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str("{{ random_uint64(start=49152, end=65535) }}", &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str("{{ random_uint64(start=4294967296) }}", &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str("{{ random_uint64(end=65535) }}", &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_uint64() }}", &context)
///     .unwrap();
/// ```
pub fn random_uint64(args: &HashMap<String, Value>) -> Result<Value> {
    parse_range_and_gen_value_in_range(args, u64::MIN, u64::MAX)
}

/// A Tera function to generate a random signed 32-bit integer.
///
/// The `start` parameter takes a signed 32-bit integer to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `i32::MIN`.
///
/// The `end` parameter also takes an signed 32-bit integer indicating the end of the range,
/// which is also inclusive. An inclusive range allows `i32::MAX` to be sampled where an
/// exclusive range does not. If `end` is not passed in, it defaults to `i32::MAX`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither in order to
/// sample across the entire `i32` space.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_int32;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_int32", random_int32);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str("{{ random_int32(start=-128, end=127) }}", &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str("{{ random_int32(start=1) }}", &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str("{{ random_int32(end=-1) }}", &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_int32() }}", &context)
///     .unwrap();
/// ```
pub fn random_int32(args: &HashMap<String, Value>) -> Result<Value> {
    parse_range_and_gen_value_in_range(args, i32::MIN, i32::MAX)
}

/// A Tera function to generate a random signed 64-bit integer.
///
/// The `start` parameter takes a signed 64-bit integer to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `i64::MIN`.
///
/// The `end` parameter also takes an signed 64-bit integer indicating the end of the range,
/// which is also inclusive. An inclusive range allows `i64::MAX` to be sampled where an
/// exclusive range does not. If `end` is not passed in, it defaults to `i64::MAX`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither in order to
/// sample across the entire `i64` space.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_int64;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_int64", random_int64);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str("{{ random_int64(start=-128, end=127) }}", &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str("{{ random_int64(start=-1) }}", &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str("{{ random_int64(end=0) }}", &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_int64() }}", &context)
///     .unwrap();
/// ```
pub fn random_int64(args: &HashMap<String, Value>) -> Result<Value> {
    parse_range_and_gen_value_in_range(args, i64::MIN, i64::MAX)
}

/// A Tera function to generate a random 32-bit float.
///
/// By default, it generates a float between `0.0` and `1.0`.
///
/// The `start` parameter takes a 32-bit float to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `0.0`.
///
/// The `end` parameter also takes a 32-bit float indicating the end of the range,
/// which is inclusive in order to remain consistent with the rest of the Tera functions.
/// If `end` is not passed in, it defaults to `1.0`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_float32;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_float32", random_float32);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str("{{ random_float32(start=-4096.0, end=4096.0) }}", &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str("{{ random_float32(start=0.0) }}", &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str("{{ random_float32(end=0.0) }}", &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_float32() }}", &context)
///     .unwrap();
/// ```
pub fn random_float32(args: &HashMap<String, Value>) -> Result<Value> {
    parse_range_and_gen_value_in_range(args, 0.0, 1.0)
}

/// A Tera function to generate a random 64-bit float.
///
/// By default, it generates a float between `0.0` and `1.0`.
///
/// The `start` parameter takes a 64-bit float to indicate the beginning of the
/// range (inclusive). If `start` is not passed in, it defaults to `0.0`.
///
/// The `end` parameter also takes a 64-bit float indicating the end of the range,
/// which is inclusive in order to remain consistent with the rest of the Tera functions.
/// If `end` is not passed in, it defaults to `1.0`.
///
/// It is possible to pass in both `start` and `end`, just one of them, or neither.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_float64;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_float64", random_float64);
/// let context: Context = Context::new();
///
/// // bound by both start and end
/// let rendered: String = tera
///     .render_str("{{ random_float64(start=-4096.0, end=4096.0) }}", &context)
///     .unwrap();
/// // bound by just start
/// let rendered: String = tera
///     .render_str("{{ random_float64(start=0.0) }}", &context)
///     .unwrap();
/// // bound by just end
/// let rendered: String = tera
///     .render_str("{{ random_float64(end=0.0) }}", &context)
///     .unwrap();
/// // bound by neither start nor end
/// let rendered: String = tera
///     .render_str("{{ random_float64() }}", &context)
///     .unwrap();
/// ```
pub fn random_float64(args: &HashMap<String, Value>) -> Result<Value> {
    parse_range_and_gen_value_in_range(args, 0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use crate::common::tests::test_tera_rand_function;
    use crate::primitives::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_random_bool() {
        test_tera_rand_function(
            random_bool,
            "random_bool",
            r#"{ "some_field": {{ random_bool() }} }"#,
            r#"\{ "some_field": (true|false) }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_char() {
        test_tera_rand_function(
            random_char,
            "random_char",
            r#"{ "some_field": {{ random_char() }} }"#,
            r#"\{ "some_field": . }"#,
        );
    }

    // uint32
    #[test]
    #[traced_test]
    fn test_random_uint32() {
        test_tera_rand_function(
            random_uint32,
            "random_uint32",
            r#"{ "some_field": {{ random_uint32() }} }"#,
            r#"\{ "some_field": \d+ }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_uint32_near_min() {
        test_tera_rand_function(
            random_uint32,
            "random_uint32",
            r#"{ "some_field": {{ random_uint32(start=0, end=2) }} }"#,
            r#"\{ "some_field": 0|1|2 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_uint32_near_max() {
        test_tera_rand_function(
            random_uint32,
            "random_uint32",
            r#"{ "some_field": {{ random_uint32(start=4294967293, end=4294967295) }} }"#,
            r#"\{ "some_field": 4294967293|4294967294|4294967295 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_uint32_with_start_only() {
        test_tera_rand_function(
            random_uint32,
            "random_uint32",
            r#"{ "some_field": {{ random_uint32(start=4294967293) }} }"#,
            r#"\{ "some_field": 4294967293|4294967294|4294967295 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_uint32_with_end_only() {
        test_tera_rand_function(
            random_uint32,
            "random_uint32",
            r#"{ "some_field": {{ random_uint32(end=2) }} }"#,
            r#"\{ "some_field": 0|1|2 }"#,
        );
    }

    // uint64
    #[test]
    #[traced_test]
    fn test_random_uint64() {
        test_tera_rand_function(
            random_uint64,
            "random_uint64",
            r#"{ "some_field": {{ random_uint64() }} }"#,
            r#"\{ "some_field": \d+ }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_uint64_near_min() {
        test_tera_rand_function(
            random_uint64,
            "random_uint64",
            r#"{ "some_field": {{ random_uint64(start=0, end=2) }} }"#,
            r#"\{ "some_field": 0|1|2 }"#,
        );
    }

    // TODO: unignore once Tera supports u64s in functions: https://github.com/Keats/tera/issues/851
    #[test]
    #[ignore]
    #[traced_test]
    fn test_random_uint64_near_max() {
        test_tera_rand_function(
            random_uint64,
            "random_uint64",
            r#"{ "some_field": {{ random_uint64(start=18446744073709551601, end=18446744073709551603) }} }"#,
            r#"\{ "some_field": 18446744073709551601|18446744073709551602|18446744073709551603 }"#,
        );
    }

    // TODO: unignore once Tera supports u64s in functions: https://github.com/Keats/tera/issues/851
    #[test]
    #[ignore]
    #[traced_test]
    fn test_random_uint64_with_start_only() {
        test_tera_rand_function(
            random_uint64,
            "random_uint64",
            r#"{ "some_field": {{ random_uint64(start=18446744073709551601) }} }"#,
            r#"\{ "some_field": 18446744073709551601|18446744073709551602|18446744073709551603 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_uint64_with_end_only() {
        test_tera_rand_function(
            random_uint64,
            "random_uint64",
            r#"{ "some_field": {{ random_uint64(end=2) }} }"#,
            r#"\{ "some_field": 0|1|2 }"#,
        );
    }

    // int32
    #[test]
    #[traced_test]
    fn test_random_int32() {
        test_tera_rand_function(
            random_int32,
            "random_int32",
            r#"{ "some_field": {{ random_int32() }} }"#,
            r#"\{ "some_field": -?\d+ }"#,
        );
    }

    // int32 in range
    #[test]
    #[traced_test]
    fn test_random_int32_near_min() {
        test_tera_rand_function(
            random_int32,
            "random_int32",
            r#"{ "some_field": {{ random_int32(start=-2147483648, end=-2147483646) }} }"#,
            r#"\{ "some_field": (-2147483648|-2147483647|-2147483646) }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int32_near_max() {
        test_tera_rand_function(
            random_int32,
            "random_int32",
            r#"{ "some_field": {{ random_int32(start=2147483645, end=2147483647) }} }"#,
            r#"\{ "some_field": 2147483645|2147483646|2147483647 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int32_with_start_only() {
        test_tera_rand_function(
            random_int32,
            "random_int32",
            r#"{ "some_field": {{ random_int32(start=2147483645) }} }"#,
            r#"\{ "some_field": 2147483645|2147483646|2147483647 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int32_with_end_only() {
        test_tera_rand_function(
            random_int32,
            "random_int32",
            r#"{ "some_field": {{ random_int32(end=-2147483646) }} }"#,
            r#"\{ "some_field": (-2147483648|-2147483647|-2147483646) }"#,
        );
    }

    // int64
    #[test]
    #[traced_test]
    fn test_random_int64() {
        test_tera_rand_function(
            random_int64,
            "random_int64",
            r#"{ "some_field": {{ random_int64() }} }"#,
            r#"\{ "some_field": -?\d+ }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int64_near_min() {
        test_tera_rand_function(
            random_int64,
            "random_int64",
            r#"{ "some_field": {{ random_int64(start=-9223372036854775808, end=-9223372036854775806) }} }"#,
            r#"\{ "some_field": (-9223372036854775808|-9223372036854775807|-9223372036854775806) }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int64_near_max() {
        test_tera_rand_function(
            random_int64,
            "random_int64",
            r#"{ "some_field": {{ random_int64(start=9223372036854775805, end=9223372036854775807) }} }"#,
            r#"\{ "some_field": 9223372036854775805|9223372036854775806|9223372036854775807 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int64_with_start_only() {
        test_tera_rand_function(
            random_int64,
            "random_int64",
            r#"{ "some_field": {{ random_int64(start=9223372036854775805) }} }"#,
            r#"\{ "some_field": 9223372036854775805|9223372036854775806|9223372036854775807 }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_int64_with_end_only() {
        test_tera_rand_function(
            random_int64,
            "random_int64",
            r#"{ "some_field": {{ random_int64(end=-9223372036854775806) }} }"#,
            r#"\{ "some_field": (-9223372036854775808|-9223372036854775807|-9223372036854775806) }"#,
        );
    }

    // float32
    #[test]
    #[traced_test]
    fn test_random_float32() {
        test_tera_rand_function(
            random_float32,
            "random_float32",
            r#"{ "some_field": {{ random_float32(start=-6.0, end=-5.0) }} }"#,
            r#"\{ "some_field": -5\.\d+ }"#,
        );
    }

    // float64
    #[test]
    #[traced_test]
    fn test_random_float64() {
        test_tera_rand_function(
            random_float64,
            "random_float64",
            r#"{ "some_field": {{ random_float64(start=-6.0, end=-5.0) }} }"#,
            r#"\{ "some_field": -5\.\d+ }"#,
        );
    }
}
