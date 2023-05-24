// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_global_tables::_list_global_tables_output::ListGlobalTablesOutputBuilder;

pub use crate::operation::list_global_tables::_list_global_tables_input::ListGlobalTablesInputBuilder;

/// Fluent builder constructing a request to `ListGlobalTables`.
///
/// <p>Lists all global tables that have a replica in the specified Region.</p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html">Version 2017.11.29 (Legacy)</a> of global tables. We recommend using <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> when creating new global tables, as it provides greater flexibility, higher efficiency and consumes less write capacity than 2017.11.29 (Legacy). To determine which version you are using, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.DetermineVersion.html">Determining the version</a>. To update existing global tables from version 2017.11.29 (Legacy) to version 2019.11.21 (Current), see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_upgrade.html"> Updating global tables</a>. </p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListGlobalTablesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder,
}
impl ListGlobalTablesFluentBuilder {
    /// Creates a new `ListGlobalTables`.
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
            crate::operation::list_global_tables::ListGlobalTables,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_global_tables::ListGlobalTablesError,
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
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_global_tables::ListGlobalTablesError,
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
    /// <p>The first global table name that this operation will evaluate.</p>
    pub fn exclusive_start_global_table_name(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.exclusive_start_global_table_name(input.into());
        self
    }
    /// <p>The first global table name that this operation will evaluate.</p>
    pub fn set_exclusive_start_global_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_exclusive_start_global_table_name(input);
        self
    }
    /// <p>The maximum number of table names to return, if the parameter is not specified DynamoDB defaults to 100.</p>
    /// <p>If the number of global tables DynamoDB finds reaches this limit, it stops the operation and returns the table names collected up to that point, with a table name in the <code>LastEvaluatedGlobalTableName</code> to apply in a subsequent operation to the <code>ExclusiveStartGlobalTableName</code> parameter.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of table names to return, if the parameter is not specified DynamoDB defaults to 100.</p>
    /// <p>If the number of global tables DynamoDB finds reaches this limit, it stops the operation and returns the table names collected up to that point, with a table name in the <code>LastEvaluatedGlobalTableName</code> to apply in a subsequent operation to the <code>ExclusiveStartGlobalTableName</code> parameter.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>Lists the global tables in a specific Region.</p>
    pub fn region_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.region_name(input.into());
        self
    }
    /// <p>Lists the global tables in a specific Region.</p>
    pub fn set_region_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_region_name(input);
        self
    }
}