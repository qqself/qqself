// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_replication_group_member_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateReplicationGroupMemberAction,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.region_name {
        object.key("RegionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_master_key_id {
        object.key("KMSMasterKeyId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.provisioned_throughput_override {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ProvisionedThroughputOverride").start_object();
        crate::protocol_serde::shape_provisioned_throughput_override::ser_provisioned_throughput_override(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.global_secondary_indexes {
        let mut array_6 = object.key("GlobalSecondaryIndexes").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_replica_global_secondary_index::ser_replica_global_secondary_index(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.table_class_override {
        object.key("TableClassOverride").string(var_9.as_str());
    }
    Ok(())
}
