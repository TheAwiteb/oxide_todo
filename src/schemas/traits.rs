use utoipa::ToSchema;

/// A trait that can be implemented to provide an example for a schema
pub trait OpenApiExample {
    /// A function that returns a serde_json::Value that can be used as an example
    fn example() -> serde_json::Value;
}

impl<T> OpenApiExample for T
where
    T: serde::Serialize + Default + for<'a> ToSchema<'a>,
{
    fn example() -> serde_json::Value {
        serde_json::to_value(Self::default()).expect("Failed to convert to JSON")
    }
}
