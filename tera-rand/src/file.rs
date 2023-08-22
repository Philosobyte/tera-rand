use crate::common::parse_arg;
use crate::error::{empty_file, internal_error, missing_arg, read_file_error};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tera::{to_value, Result, Value};

lazy_static! {
    static ref FILE_CACHE: DashMap<String, Vec<String>> = DashMap::new();
}

/// A Tera function to sample a random value from a line-delimited file of strings. The filepath
/// should be passed in as an argument to the `path` parameter.
///
/// Note that the contents of the filepath is read only once and cached.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_from_file;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_from_file", random_from_file);
/// let context: Context = Context::new();
///
/// let rendered: String = tera
///     .render_str(r#"{{ random_from_file(path="resources/test/addresses.txt") }}"#, &context)
///     .unwrap();
/// ```
pub fn random_from_file(args: &HashMap<String, Value>) -> Result<Value> {
    let filepath_opt: Option<String> = parse_arg(args, "random_from_file", "path")?;
    let filepath: String = filepath_opt.ok_or_else(|| missing_arg("random_from_file", "path"))?;

    // read the file only if we haven't read it before
    if !FILE_CACHE.contains_key(&filepath) {
        let input_file: File = File::open(&filepath)
            .map_err(|source| read_file_error("random_from_file", filepath.clone(), source))?;
        let buf_reader: BufReader<File> = BufReader::new(input_file);

        let mut file_values: Vec<String> = Vec::new();
        for line_result in buf_reader.lines() {
            let line: String = line_result
                .map_err(|source| read_file_error("random_from_file", filepath.clone(), source))?;
            file_values.push(line);
        }

        if file_values.is_empty() {
            return Err(empty_file("random_from_file", filepath));
        }
        FILE_CACHE.insert(filepath.clone(), file_values);
    }
    let possible_values_opt: Option<Ref<String, Vec<String>>> = FILE_CACHE.get(&filepath);

    match possible_values_opt {
        Some(reference) => {
            let possible_values: &Vec<String> = reference.value();
            let index_to_sample: usize = thread_rng().gen_range(0usize..possible_values.len());

            match possible_values.get(index_to_sample) {
                Some(sampled_value) => {
                    let json_value = to_value(sampled_value)?;
                    Ok(json_value)
                }
                None => Err(internal_error(format!(
                    "Unable to sample value with line number {} for file at path {}",
                    index_to_sample, filepath
                ))),
            }
        }
        None => Err(internal_error(format!(
            "File cache did not contain an entry for file {filepath}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::common::tests::test_tera_rand_function;
    use crate::file::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_random_from_file() {
        test_tera_rand_function(
            random_from_file,
            "random_from_file",
            r#"{ "some_field": "{{ random_from_file(path="resources/test/days.txt") }}" }"#,
            r#"\{ "some_field": "(Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday)" }"#,
        )
    }
}
