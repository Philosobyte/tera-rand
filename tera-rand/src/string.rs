use crate::common::parse_arg;
use crate::error::unsupported_arg;
use rand::distributions::{Alphanumeric, DistString, Standard};
use rand::thread_rng;
use std::collections::HashMap;
use tera::{to_value, Result, Value};

/// A Tera function to generate a random String.
///
/// By default, this function will generate an alphanumeric string of length 8. For a string with
/// a different length, pass an integer length to the `length` parameter in the template.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_string;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_string", random_string);
/// let context: Context = Context::new();
///
/// // use the default length of 8
/// let rendered: String = tera
///     .render_str("{{ random_string() }}", &context)
///     .unwrap();
/// // use a custom length of 16
/// let rendered: String = tera
///     .render_str("{{ random_string(length=16) }}", &context)
///     .unwrap();
/// // use alphanumeric space (which is also the default)
/// let rendered: String = tera
///     .render_str(r#"{{ random_string(space="alphanumeric") }}"#, &context)
///     .unwrap();
/// // use standard space
/// let rendered: String = tera
///     .render_str(r#"{{ random_string(space="standard") }}"#, &context)
///     .unwrap();
/// ```
pub fn random_string(args: &HashMap<String, Value>) -> Result<Value> {
    let str_length: usize = parse_arg(args, "length")?.unwrap_or(8usize);

    let space_as_string: String =
        parse_arg(args, "space")?.unwrap_or_else(|| String::from("alphanumeric"));

    let random_string: String = match space_as_string.as_str() {
        "alphanumeric" => Ok(Alphanumeric.sample_string(&mut thread_rng(), str_length)),
        "standard" => Ok(Standard.sample_string(&mut thread_rng(), str_length)),
        _ => Err(unsupported_arg("space", space_as_string)),
    }?;
    let json_value: Value = to_value(random_string)?;
    Ok(json_value)
}

#[cfg(test)]
mod tests {
    use crate::common::tests::test_tera_rand_function;
    use crate::string::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_random_string() {
        test_tera_rand_function(
            random_string,
            "random_string",
            r#"{ "some_field": "{{ random_string() }}" }"#,
            r#"\{ "some_field": "[\w\d]{8}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_string_with_custom_length() {
        test_tera_rand_function(
            random_string,
            "random_string",
            r#"{ "some_field": "{{ random_string(length=12) }}" }"#,
            r#"\{ "some_field": "[\w\d]{12}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_string_with_alphanumeric_space() {
        test_tera_rand_function(
            random_string,
            "random_string",
            r#"{ "some_field": "{{ random_string(space="alphanumeric") }}" }"#,
            r#"\{ "some_field": "[\w\d]{8}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_string_with_standard_space() {
        test_tera_rand_function(
            random_string,
            "random_string",
            r#"{ "some_field": "{{ random_string(space="standard") }}" }"#,
            r#"\{ "some_field": ".{8}" }"#,
        );
    }

    #[test]
    #[traced_test]
    fn test_random_string_with_standard_space_and_custom_length() {
        test_tera_rand_function(
            random_string,
            "random_string",
            r#"{ "some_field": "{{ random_string(space="standard", length=12) }}" }"#,
            r#"\{ "some_field": ".{12}" }"#,
        );
    }
}
