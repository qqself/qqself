// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_global_table_global_secondary_index_settings_update(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_name {
        object.key("IndexName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provisioned_write_capacity_units {
        object.key("ProvisionedWriteCapacityUnits").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.provisioned_write_capacity_auto_scaling_settings_update {
        #[allow(unused_mut)]
        let mut object_4 = object
            .key("ProvisionedWriteCapacityAutoScalingSettingsUpdate")
            .start_object();
        crate::protocol_serde::shape_auto_scaling_settings_update::ser_auto_scaling_settings_update(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
