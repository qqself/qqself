// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_global_table_settings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.global_table_name {
        object.key("GlobalTableName").string(var_1.as_str());
    }
    Ok(())
}
