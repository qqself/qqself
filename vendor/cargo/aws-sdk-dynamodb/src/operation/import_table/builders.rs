// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_table::_import_table_output::ImportTableOutputBuilder;

pub use crate::operation::import_table::_import_table_input::ImportTableInputBuilder;

/// Fluent builder constructing a request to `ImportTable`.
///
/// <p> Imports table data from an S3 bucket. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportTableFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_table::builders::ImportTableInputBuilder,
}
impl ImportTableFluentBuilder {
    /// Creates a new `ImportTable`.
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
            crate::operation::import_table::ImportTable,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::import_table::ImportTableError>,
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
        crate::operation::import_table::ImportTableOutput,
        aws_smithy_http::result::SdkError<crate::operation::import_table::ImportTableError>,
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
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p> The S3 bucket that provides the source for the import. </p>
    pub fn s3_bucket_source(mut self, input: crate::types::S3BucketSource) -> Self {
        self.inner = self.inner.s3_bucket_source(input);
        self
    }
    /// <p> The S3 bucket that provides the source for the import. </p>
    pub fn set_s3_bucket_source(
        mut self,
        input: std::option::Option<crate::types::S3BucketSource>,
    ) -> Self {
        self.inner = self.inner.set_s3_bucket_source(input);
        self
    }
    /// <p> The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>. </p>
    pub fn input_format(mut self, input: crate::types::InputFormat) -> Self {
        self.inner = self.inner.input_format(input);
        self
    }
    /// <p> The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>. </p>
    pub fn set_input_format(
        mut self,
        input: std::option::Option<crate::types::InputFormat>,
    ) -> Self {
        self.inner = self.inner.set_input_format(input);
        self
    }
    /// <p> Additional properties that specify how the input is formatted, </p>
    pub fn input_format_options(mut self, input: crate::types::InputFormatOptions) -> Self {
        self.inner = self.inner.input_format_options(input);
        self
    }
    /// <p> Additional properties that specify how the input is formatted, </p>
    pub fn set_input_format_options(
        mut self,
        input: std::option::Option<crate::types::InputFormatOptions>,
    ) -> Self {
        self.inner = self.inner.set_input_format_options(input);
        self
    }
    /// <p> Type of compression to be used on the input coming from the imported table. </p>
    pub fn input_compression_type(mut self, input: crate::types::InputCompressionType) -> Self {
        self.inner = self.inner.input_compression_type(input);
        self
    }
    /// <p> Type of compression to be used on the input coming from the imported table. </p>
    pub fn set_input_compression_type(
        mut self,
        input: std::option::Option<crate::types::InputCompressionType>,
    ) -> Self {
        self.inner = self.inner.set_input_compression_type(input);
        self
    }
    /// <p>Parameters for the table to import the data into. </p>
    pub fn table_creation_parameters(
        mut self,
        input: crate::types::TableCreationParameters,
    ) -> Self {
        self.inner = self.inner.table_creation_parameters(input);
        self
    }
    /// <p>Parameters for the table to import the data into. </p>
    pub fn set_table_creation_parameters(
        mut self,
        input: std::option::Option<crate::types::TableCreationParameters>,
    ) -> Self {
        self.inner = self.inner.set_table_creation_parameters(input);
        self
    }
}
