use std::collections::HashMap;
use tera::{to_value, Result, Value};
use uuid::Uuid;

/// A Tera function to generate a random UUIDv4.
///
/// # Example usage
///
/// ```edition2021
/// use tera::{Context, Tera};
/// use tera_rand::random_uuid;
///
/// let mut tera: Tera = Tera::default();
/// tera.register_function("random_uuid", random_uuid);
///
/// let context: Context = Context::new();
/// let rendered: String = tera.render_str("{{ random_uuid() }}", &context).unwrap();
/// ```
#[cfg(feature = "uuid")]
pub fn random_uuid(_args: &HashMap<String, Value>) -> Result<Value> {
    let random_uuid: Uuid = Uuid::new_v4();
    let json_value: Value = to_value(random_uuid.to_string())?;
    Ok(json_value)
}

#[cfg(test)]
mod tests {
    use crate::common::tests::test_tera_rand_function;
    use crate::random_uint32;
    use crate::uuid::*;
    use tera::{Context, Tera};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    #[cfg(feature = "uuid")]
    fn test_random_uuid() {
        test_tera_rand_function(
            random_uuid,
            "random_uuid",
            r#"{ "some_field": "{{ random_uuid() }}" }"#,
            r#"\{ "some_field": "[\da-f]{8}-([\da-f]{4}-){3}[\da-f]{12}" }"#,
        );
    }

    fn test_stuff() {
        let mut tera: Tera = Tera::default();
        tera.register_function("random_uint32", random_uint32);
        let context: Context = Context::new();
        // bound by both start and end
        let result: String = tera
            .render_str("{{ random_uint32(start=49152, end=65535) }}", &context)
            .unwrap();
        // bound by just start
        let result: String = tera
            .render_str("{{ random_uint32(start=4294967290) }}", &context)
            .unwrap();
        // bound by just end
        let result: String = tera
            .render_str("{{ random_uint32(end=65535) }}", &context)
            .unwrap();
        // bound by neither start nor end
        let result: String = tera.render_str("{{ random_uint32() }}", &context).unwrap();
    }
}
