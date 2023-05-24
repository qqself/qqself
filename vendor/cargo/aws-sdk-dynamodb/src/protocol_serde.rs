// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_batch_execute_statement;

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &http::HeaderMap,
    response_body: &[u8],
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_batch_get_item;

pub(crate) mod shape_batch_write_item;

pub(crate) mod shape_create_backup;

pub(crate) mod shape_create_global_table;

pub(crate) mod shape_create_table;

pub(crate) mod shape_delete_backup;

pub(crate) mod shape_delete_item;

pub(crate) mod shape_delete_table;

pub(crate) mod shape_describe_backup;

pub(crate) mod shape_describe_continuous_backups;

pub(crate) mod shape_describe_contributor_insights;

pub(crate) mod shape_describe_endpoints;

pub(crate) mod shape_describe_export;

pub(crate) mod shape_describe_global_table;

pub(crate) mod shape_describe_global_table_settings;

pub(crate) mod shape_describe_import;

pub(crate) mod shape_describe_kinesis_streaming_destination;

pub(crate) mod shape_describe_limits;

pub(crate) mod shape_describe_table;

pub(crate) mod shape_describe_table_replica_auto_scaling;

pub(crate) mod shape_describe_time_to_live;

pub(crate) mod shape_disable_kinesis_streaming_destination;

pub(crate) mod shape_enable_kinesis_streaming_destination;

pub(crate) mod shape_execute_statement;

pub(crate) mod shape_execute_transaction;

pub(crate) mod shape_export_table_to_point_in_time;

pub(crate) mod shape_get_item;

pub(crate) mod shape_import_table;

pub(crate) mod shape_list_backups;

pub(crate) mod shape_list_contributor_insights;

pub(crate) mod shape_list_exports;

pub(crate) mod shape_list_global_tables;

pub(crate) mod shape_list_imports;

pub(crate) mod shape_list_tables;

pub(crate) mod shape_list_tags_of_resource;

pub(crate) mod shape_put_item;

pub(crate) mod shape_query;

pub(crate) mod shape_restore_table_from_backup;

pub(crate) mod shape_restore_table_to_point_in_time;

pub(crate) mod shape_scan;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_transact_get_items;

pub(crate) mod shape_transact_write_items;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_continuous_backups;

pub(crate) mod shape_update_contributor_insights;

pub(crate) mod shape_update_global_table;

pub(crate) mod shape_update_global_table_settings;

pub(crate) mod shape_update_item;

pub(crate) mod shape_update_table;

pub(crate) mod shape_update_table_replica_auto_scaling;

pub(crate) mod shape_update_time_to_live;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_backup_in_use_exception;

pub(crate) mod shape_backup_not_found_exception;

pub(crate) mod shape_batch_execute_statement_input;

pub(crate) mod shape_batch_get_item_input;

pub(crate) mod shape_batch_write_item_input;

pub(crate) mod shape_conditional_check_failed_exception;

pub(crate) mod shape_continuous_backups_unavailable_exception;

pub(crate) mod shape_create_backup_input;

pub(crate) mod shape_create_global_table_input;

pub(crate) mod shape_create_table_input;

pub(crate) mod shape_delete_backup_input;

pub(crate) mod shape_delete_item_input;

pub(crate) mod shape_delete_table_input;

pub(crate) mod shape_describe_backup_input;

pub(crate) mod shape_describe_continuous_backups_input;

pub(crate) mod shape_describe_contributor_insights_input;

pub(crate) mod shape_describe_export_input;

pub(crate) mod shape_describe_global_table_input;

pub(crate) mod shape_describe_global_table_settings_input;

pub(crate) mod shape_describe_import_input;

pub(crate) mod shape_describe_kinesis_streaming_destination_input;

pub(crate) mod shape_describe_table_input;

pub(crate) mod shape_describe_table_replica_auto_scaling_input;

pub(crate) mod shape_describe_time_to_live_input;

pub(crate) mod shape_disable_kinesis_streaming_destination_input;

pub(crate) mod shape_duplicate_item_exception;

pub(crate) mod shape_enable_kinesis_streaming_destination_input;

pub(crate) mod shape_execute_statement_input;

pub(crate) mod shape_execute_transaction_input;

pub(crate) mod shape_export_conflict_exception;

pub(crate) mod shape_export_not_found_exception;

pub(crate) mod shape_export_table_to_point_in_time_input;

pub(crate) mod shape_get_item_input;

pub(crate) mod shape_global_table_already_exists_exception;

pub(crate) mod shape_global_table_not_found_exception;

pub(crate) mod shape_idempotent_parameter_mismatch_exception;

pub(crate) mod shape_import_conflict_exception;

pub(crate) mod shape_import_not_found_exception;

pub(crate) mod shape_import_table_input;

pub(crate) mod shape_index_not_found_exception;

pub(crate) mod shape_internal_server_error;

pub(crate) mod shape_invalid_endpoint_exception;

pub(crate) mod shape_invalid_export_time_exception;

pub(crate) mod shape_invalid_restore_time_exception;

pub(crate) mod shape_item_collection_size_limit_exceeded_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_backups_input;

pub(crate) mod shape_list_contributor_insights_input;

pub(crate) mod shape_list_exports_input;

pub(crate) mod shape_list_global_tables_input;

pub(crate) mod shape_list_imports_input;

pub(crate) mod shape_list_tables_input;

pub(crate) mod shape_list_tags_of_resource_input;

pub(crate) mod shape_point_in_time_recovery_unavailable_exception;

pub(crate) mod shape_provisioned_throughput_exceeded_exception;

pub(crate) mod shape_put_item_input;

pub(crate) mod shape_query_input;

pub(crate) mod shape_replica_already_exists_exception;

pub(crate) mod shape_replica_not_found_exception;

pub(crate) mod shape_request_limit_exceeded;

pub(crate) mod shape_resource_in_use_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_restore_table_from_backup_input;

pub(crate) mod shape_restore_table_to_point_in_time_input;

pub(crate) mod shape_scan_input;

pub(crate) mod shape_table_already_exists_exception;

pub(crate) mod shape_table_in_use_exception;

pub(crate) mod shape_table_not_found_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_transact_get_items_input;

pub(crate) mod shape_transact_write_items_input;

pub(crate) mod shape_transaction_canceled_exception;

pub(crate) mod shape_transaction_conflict_exception;

pub(crate) mod shape_transaction_in_progress_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_continuous_backups_input;

pub(crate) mod shape_update_contributor_insights_input;

pub(crate) mod shape_update_global_table_input;

pub(crate) mod shape_update_global_table_settings_input;

pub(crate) mod shape_update_item_input;

pub(crate) mod shape_update_table_input;

pub(crate) mod shape_update_table_replica_auto_scaling_input;

pub(crate) mod shape_update_time_to_live_input;

pub(crate) mod shape_attribute_definition;

pub(crate) mod shape_attribute_map;

pub(crate) mod shape_attribute_value;

pub(crate) mod shape_attribute_value_update;

pub(crate) mod shape_auto_scaling_settings_update;

pub(crate) mod shape_backup_description;

pub(crate) mod shape_backup_details;

pub(crate) mod shape_backup_summaries;

pub(crate) mod shape_batch_get_request_map;

pub(crate) mod shape_batch_get_response_map;

pub(crate) mod shape_batch_statement_request;

pub(crate) mod shape_batch_write_item_request_map;

pub(crate) mod shape_cancellation_reason_list;

pub(crate) mod shape_condition;

pub(crate) mod shape_consumed_capacity;

pub(crate) mod shape_consumed_capacity_multiple;

pub(crate) mod shape_continuous_backups_description;

pub(crate) mod shape_contributor_insights_rule_list;

pub(crate) mod shape_contributor_insights_summaries;

pub(crate) mod shape_endpoints;

pub(crate) mod shape_expected_attribute_value;

pub(crate) mod shape_export_description;

pub(crate) mod shape_export_summaries;

pub(crate) mod shape_failure_exception;

pub(crate) mod shape_global_secondary_index;

pub(crate) mod shape_global_secondary_index_auto_scaling_update;

pub(crate) mod shape_global_secondary_index_update;

pub(crate) mod shape_global_table_description;

pub(crate) mod shape_global_table_global_secondary_index_settings_update;

pub(crate) mod shape_global_table_list;

pub(crate) mod shape_import_summary_list;

pub(crate) mod shape_import_table_description;

pub(crate) mod shape_input_format_options;

pub(crate) mod shape_item_collection_metrics;

pub(crate) mod shape_item_collection_metrics_per_table;

pub(crate) mod shape_item_list;

pub(crate) mod shape_item_response_list;

pub(crate) mod shape_key;

pub(crate) mod shape_key_schema_element;

pub(crate) mod shape_keys_and_attributes;

pub(crate) mod shape_kinesis_data_stream_destinations;

pub(crate) mod shape_local_secondary_index;

pub(crate) mod shape_parameterized_statement;

pub(crate) mod shape_parti_ql_batch_response;

pub(crate) mod shape_point_in_time_recovery_specification;

pub(crate) mod shape_provisioned_throughput;

pub(crate) mod shape_replica;

pub(crate) mod shape_replica_auto_scaling_update;

pub(crate) mod shape_replica_settings_description_list;

pub(crate) mod shape_replica_settings_update;

pub(crate) mod shape_replica_update;

pub(crate) mod shape_replication_group_update;

pub(crate) mod shape_s3_bucket_source;

pub(crate) mod shape_sse_specification;

pub(crate) mod shape_stream_specification;

pub(crate) mod shape_table_auto_scaling_description;

pub(crate) mod shape_table_creation_parameters;

pub(crate) mod shape_table_description;

pub(crate) mod shape_table_name_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_time_to_live_description;

pub(crate) mod shape_time_to_live_specification;

pub(crate) mod shape_transact_get_item;

pub(crate) mod shape_transact_write_item;

pub(crate) mod shape_write_request;

pub(crate) mod shape_archival_summary;

pub(crate) mod shape_attribute_definitions;

pub(crate) mod shape_auto_scaling_policy_update;

pub(crate) mod shape_backup_summary;

pub(crate) mod shape_batch_statement_response;

pub(crate) mod shape_billing_mode_summary;

pub(crate) mod shape_cancellation_reason;

pub(crate) mod shape_capacity;

pub(crate) mod shape_condition_check;

pub(crate) mod shape_contributor_insights_summary;

pub(crate) mod shape_create_global_secondary_index_action;

pub(crate) mod shape_create_replica_action;

pub(crate) mod shape_create_replication_group_member_action;

pub(crate) mod shape_csv_options;

pub(crate) mod shape_delete;

pub(crate) mod shape_delete_global_secondary_index_action;

pub(crate) mod shape_delete_replica_action;

pub(crate) mod shape_delete_replication_group_member_action;

pub(crate) mod shape_delete_request;

pub(crate) mod shape_endpoint;

pub(crate) mod shape_export_summary;

pub(crate) mod shape_get;

pub(crate) mod shape_global_secondary_index_description_list;

pub(crate) mod shape_global_table;

pub(crate) mod shape_import_summary;

pub(crate) mod shape_item_collection_key_attribute_map;

pub(crate) mod shape_item_collection_metrics_multiple;

pub(crate) mod shape_item_collection_size_estimate_range;

pub(crate) mod shape_item_response;

pub(crate) mod shape_key_schema;

pub(crate) mod shape_kinesis_data_stream_destination;

pub(crate) mod shape_local_secondary_index_description_list;

pub(crate) mod shape_point_in_time_recovery_description;

pub(crate) mod shape_projection;

pub(crate) mod shape_provisioned_throughput_description;

pub(crate) mod shape_put;

pub(crate) mod shape_put_request;

pub(crate) mod shape_replica_auto_scaling_description_list;

pub(crate) mod shape_replica_description_list;

pub(crate) mod shape_replica_global_secondary_index_auto_scaling_update;

pub(crate) mod shape_replica_global_secondary_index_settings_update;

pub(crate) mod shape_replica_settings_description;

pub(crate) mod shape_restore_summary;

pub(crate) mod shape_secondary_indexes_capacity_map;

pub(crate) mod shape_source_table_details;

pub(crate) mod shape_source_table_feature_details;

pub(crate) mod shape_sse_description;

pub(crate) mod shape_table_class_summary;

pub(crate) mod shape_update;

pub(crate) mod shape_update_global_secondary_index_action;

pub(crate) mod shape_update_replication_group_member_action;

pub(crate) mod shape_write_requests;

pub(crate) mod shape_attribute_name_list;

pub(crate) mod shape_auto_scaling_settings_description;

pub(crate) mod shape_auto_scaling_target_tracking_scaling_policy_configuration_update;

pub(crate) mod shape_batch_statement_error;

pub(crate) mod shape_binary_set_attribute_value;

pub(crate) mod shape_expression_attribute_name_map;

pub(crate) mod shape_global_secondary_index_description;

pub(crate) mod shape_global_secondary_index_list;

pub(crate) mod shape_global_secondary_indexes;

pub(crate) mod shape_key_list;

pub(crate) mod shape_list_attribute_value;

pub(crate) mod shape_local_secondary_index_description;

pub(crate) mod shape_local_secondary_indexes;

pub(crate) mod shape_map_attribute_value;

pub(crate) mod shape_number_set_attribute_value;

pub(crate) mod shape_provisioned_throughput_override;

pub(crate) mod shape_replica_auto_scaling_description;

pub(crate) mod shape_replica_description;

pub(crate) mod shape_replica_global_secondary_index;

pub(crate) mod shape_replica_global_secondary_index_settings_description_list;

pub(crate) mod shape_replica_list;

pub(crate) mod shape_string_set_attribute_value;

pub(crate) mod shape_auto_scaling_policy_description_list;

pub(crate) mod shape_csv_header_list;

pub(crate) mod shape_global_secondary_index_info;

pub(crate) mod shape_local_secondary_index_info;

pub(crate) mod shape_replica_global_secondary_index_auto_scaling_description_list;

pub(crate) mod shape_replica_global_secondary_index_description_list;

pub(crate) mod shape_replica_global_secondary_index_settings_description;

pub(crate) mod shape_auto_scaling_policy_description;

pub(crate) mod shape_non_key_attribute_name_list;

pub(crate) mod shape_put_item_input_attribute_map;

pub(crate) mod shape_replica_global_secondary_index_auto_scaling_description;

pub(crate) mod shape_replica_global_secondary_index_description;

pub(crate) mod shape_auto_scaling_target_tracking_scaling_policy_configuration_description;
