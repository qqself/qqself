// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tables::_list_tables_output::ListTablesOutputBuilder;

pub use crate::operation::list_tables::_list_tables_input::ListTablesInputBuilder;

/// Fluent builder constructing a request to `ListTables`.
///
/// <p>Returns an array of table names associated with the current account and endpoint. The output from <code>ListTables</code> is paginated, with each page returning a maximum of 100 table names.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListTablesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tables::builders::ListTablesInputBuilder,
}
impl ListTablesFluentBuilder {
    /// Creates a new `ListTables`.
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
            crate::operation::list_tables::ListTables,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_tables::ListTablesError>,
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
        crate::operation::list_tables::ListTablesOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_tables::ListTablesError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_tables::paginator::ListTablesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_tables::paginator::ListTablesPaginator {
        crate::operation::list_tables::paginator::ListTablesPaginator::new(self.handle, self.inner)
    }
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub fn exclusive_start_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.exclusive_start_table_name(input.into());
        self
    }
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub fn set_exclusive_start_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_exclusive_start_table_name(input);
        self
    }
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
}
