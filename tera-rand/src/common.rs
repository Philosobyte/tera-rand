use crate::error::arg_parse_error;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{random, thread_rng, Rng};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use tera::{from_value, to_value, Result, Value};

// Parse an argument for the given `parameter` name from `args`, a map of arguments.
//
// `function` should be the name of the calling function; this is used only for debugging purposes.
//
// If an argument is not found at all, this function returns `tera::Result::Ok(None)`. If an
// argument is found, but Tera fails to parse it, this function returns a `tera::Result::Err`.
pub(crate) fn parse_arg<T>(
    args: &HashMap<String, Value>,
    function: &'static str,
    parameter: &'static str,
) -> Result<Option<T>>
where
    T: DeserializeOwned,
{
    args.get(parameter)
        .cloned()
        .map(|length_value| from_value(length_value))
        .transpose()
        .map_err(|source| arg_parse_error(function, parameter, source))
}

// Generate a random value.
//
// If both `start_opt` and `end_opt` are provided, they will bound the space from which the value
// is sampled.
//
// If one of `start_opt` or `end_opt` is provided but the other is not, then `default_start` or
// `default_end` will fill in for the missing bound.
//
// If neither `start_opt` nor `end_opt` is provided, then this function will generate a value from
// the standard distribution, notably NOT using either `default_start` or `default_end`. This is
// done for performance.
pub(crate) fn gen_value_in_range<T>(
    start_opt: Option<T>,
    end_opt: Option<T>,
    default_start: T,
    default_end: T,
) -> T
where
    T: SampleUniform,
    RangeInclusive<T>: SampleRange<T>,
    Standard: Distribution<T>,
{
    match (start_opt, end_opt) {
        (Some(start), Some(end)) => thread_rng().gen_range(start..=end),
        (Some(start), None) => thread_rng().gen_range(start..=default_end),
        (None, Some(end)) => thread_rng().gen_range(default_start..=end),
        (None, None) => random::<T>(),
    }
}

// convenience function to parse `start` and `end` arguments from the Tera template function call,
// generate a random value in between `start` and/or `end` if specified, and then convert the
// result into a value for Tera to render.
pub(crate) fn parse_range_and_gen_value_in_range<T>(
    args: &HashMap<String, Value>,
    function: &'static str,
    default_start: T,
    default_end: T,
) -> Result<Value>
where
    T: SampleUniform + DeserializeOwned + Serialize,
    RangeInclusive<T>: SampleRange<T>,
    Standard: Distribution<T>,
{
    let start_opt: Option<T> = parse_arg(args, function, "start")?;
    let end_opt: Option<T> = parse_arg(args, function, "end")?;

    let random_value: T = gen_value_in_range(start_opt, end_opt, default_start, default_end);
    let json_value: Value = to_value(random_value)?;
    Ok(json_value)
}

#[cfg(test)]
pub(crate) mod tests {
    use regex::Regex;
    use tera::{Context, Function, Tera};
    use tracing::trace;

    pub(crate) fn test_tera_rand_function<F>(
        function: F,
        function_name: &str,
        input_template_str: &str,
        expected_regex_str: &str,
    ) where
        F: Function + 'static,
    {
        let mut tera: Tera = Tera::default();
        tera.register_function(function_name, function);

        let expected_regex: Regex = Regex::new(expected_regex_str).unwrap_or_else(|e| {
            panic!(
                "Unable to construct a Regex object out of {} due to error: {:?}",
                expected_regex_str, e
            )
        });

        let context: Context = Context::new();
        let render_result: String = tera
            .render_str(input_template_str, &context)
            .unwrap_or_else(|e| {
                panic!(
                    "Unable to render template {} for function {} due to error: {:?}",
                    input_template_str, function_name, e
                )
            });

        trace!("render result: {render_result}");
        assert!(expected_regex.is_match(render_result.as_str()));
    }
}
