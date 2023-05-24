// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RestoreTableFromBackup`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_table_name(impl Into<String>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::target_table_name) / [`set_target_table_name(Option<String>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_target_table_name): <p>The name of the new table to which the backup must be restored.</p>
    ///   - [`backup_arn(impl Into<String>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::backup_arn) / [`set_backup_arn(Option<String>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_backup_arn): <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    ///   - [`billing_mode_override(BillingMode)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::billing_mode_override) / [`set_billing_mode_override(Option<BillingMode>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_billing_mode_override): <p>The billing mode of the restored table.</p>
    ///   - [`global_secondary_index_override(Vec<GlobalSecondaryIndex>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::global_secondary_index_override) / [`set_global_secondary_index_override(Option<Vec<GlobalSecondaryIndex>>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_global_secondary_index_override): <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    ///   - [`local_secondary_index_override(Vec<LocalSecondaryIndex>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::local_secondary_index_override) / [`set_local_secondary_index_override(Option<Vec<LocalSecondaryIndex>>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_local_secondary_index_override): <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    ///   - [`provisioned_throughput_override(ProvisionedThroughput)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::provisioned_throughput_override) / [`set_provisioned_throughput_override(Option<ProvisionedThroughput>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_provisioned_throughput_override): <p>Provisioned throughput settings for the restored table.</p>
    ///   - [`sse_specification_override(SseSpecification)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::sse_specification_override) / [`set_sse_specification_override(Option<SseSpecification>)`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::set_sse_specification_override): <p>The new server-side encryption settings for the restored table.</p>
    /// - On success, responds with [`RestoreTableFromBackupOutput`](crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput) with field(s):
    ///   - [`table_description(Option<TableDescription>)`](crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput::table_description): <p>The description of the table created from an existing backup.</p>
    /// - On failure, responds with [`SdkError<RestoreTableFromBackupError>`](crate::operation::restore_table_from_backup::RestoreTableFromBackupError)
    pub fn restore_table_from_backup(
        &self,
    ) -> crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder
    {
        crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupFluentBuilder::new(self.handle.clone())
    }
}
