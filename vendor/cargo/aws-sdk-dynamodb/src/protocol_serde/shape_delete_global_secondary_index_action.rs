// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_global_secondary_index_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DeleteGlobalSecondaryIndexAction,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_name {
        object.key("IndexName").string(var_1.as_str());
    }
    Ok(())
}