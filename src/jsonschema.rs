use {
    super::*,
    ::schemars::{
        SchemaGenerator,
        JsonSchema,
        json_schema,
        Schema,
    },
    std::borrow::Cow,
};

impl<O: Identifiable + JsonSchema> JsonSchema for Id<O> {
    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!("Id<{}>", O::schema_id()))
    }
    fn schema_name() -> Cow<'static, str> {
        format!("{}_id", O::class().prefix()).into()
    }
    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
        })
    }
    fn always_inline_schema() -> bool {
        true
    }
}

impl<O: Identifiable + JsonSchema> JsonSchema for Ided<O> {
    fn schema_name() -> Cow<'static, str> {
        format!("{}_ided", O::schema_name()).into()
    }
    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!("Ided<{}>", O::schema_id()))
    }
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = O::json_schema(gen);
        schema
            .ensure_object()
            .insert("id".into(), "string".into());
        schema
    }
}
