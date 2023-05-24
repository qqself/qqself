// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_global_table::_update_global_table_output::UpdateGlobalTableOutputBuilder;

pub use crate::operation::update_global_table::_update_global_table_input::UpdateGlobalTableInputBuilder;

/// Fluent builder constructing a request to `UpdateGlobalTable`.
///
/// <p>Adds or removes replicas in the specified global table. The global table must already exist to be able to use this operation. Any replica to be added must be empty, have the same name as the global table, have the same key schema, have DynamoDB Streams enabled, and have the same provisioned and maximum write capacity units.</p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html">Version 2017.11.29 (Legacy)</a> of global tables. We recommend using <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> when creating new global tables, as it provides greater flexibility, higher efficiency and consumes less write capacity than 2017.11.29 (Legacy). To determine which version you are using, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.DetermineVersion.html">Determining the version</a>. To update existing global tables from version 2017.11.29 (Legacy) to version 2019.11.21 (Current), see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_upgrade.html"> Updating global tables</a>. </p>
/// </important> <note>
/// <p> This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html">Version 2017.11.29</a> of global tables. If you are using global tables <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21</a> you can use <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DescribeTable.html">DescribeTable</a> instead. </p>
/// <p> Although you can use <code>UpdateGlobalTable</code> to add replicas and remove replicas in a single request, for simplicity we recommend that you issue separate requests for adding or removing replicas. </p>
/// </note>
/// <p> If global secondary indexes are specified, then the following conditions must also be met: </p>
/// <ul>
/// <li> <p> The global secondary indexes must have the same name. </p> </li>
/// <li> <p> The global secondary indexes must have the same hash key and sort key (if present). </p> </li>
/// <li> <p> The global secondary indexes must have the same provisioned and maximum write capacity units. </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateGlobalTableFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_global_table::builders::UpdateGlobalTableInputBuilder,
}
impl UpdateGlobalTableFluentBuilder {
    /// Creates a new `UpdateGlobalTable`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }
    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_global_table::UpdateGlobalTable,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_global_table::UpdateGlobalTableError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_global_table::UpdateGlobalTableError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The global table name.</p>
    pub fn global_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_table_name(input.into());
        self
    }
    /// <p>The global table name.</p>
    pub fn set_global_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_table_name(input);
        self
    }
    /// Appends an item to `ReplicaUpdates`.
    ///
    /// To override the contents of this collection use [`set_replica_updates`](Self::set_replica_updates).
    ///
    /// <p>A list of Regions that should be added or removed from the global table.</p>
    pub fn replica_updates(mut self, input: crate::types::ReplicaUpdate) -> Self {
        self.inner = self.inner.replica_updates(input);
        self
    }
    /// <p>A list of Regions that should be added or removed from the global table.</p>
    pub fn set_replica_updates(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ReplicaUpdate>>,
    ) -> Self {
        self.inner = self.inner.set_replica_updates(input);
        self
    }
}
