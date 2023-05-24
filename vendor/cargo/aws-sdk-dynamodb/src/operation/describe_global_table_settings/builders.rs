// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_global_table_settings::_describe_global_table_settings_output::DescribeGlobalTableSettingsOutputBuilder;

pub use crate::operation::describe_global_table_settings::_describe_global_table_settings_input::DescribeGlobalTableSettingsInputBuilder;

/// Fluent builder constructing a request to `DescribeGlobalTableSettings`.
///
/// <p>Describes Region-specific settings for a global table.</p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html">Version 2017.11.29 (Legacy)</a> of global tables. We recommend using <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> when creating new global tables, as it provides greater flexibility, higher efficiency and consumes less write capacity than 2017.11.29 (Legacy). To determine which version you are using, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.DetermineVersion.html">Determining the version</a>. To update existing global tables from version 2017.11.29 (Legacy) to version 2019.11.21 (Current), see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_upgrade.html"> Updating global tables</a>. </p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeGlobalTableSettingsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsInputBuilder,
}
impl DescribeGlobalTableSettingsFluentBuilder {
    /// Creates a new `DescribeGlobalTableSettings`.
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
            crate::operation::describe_global_table_settings::DescribeGlobalTableSettings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError,
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
        crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError,
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
    /// <p>The name of the global table to describe.</p>
    pub fn global_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_table_name(input.into());
        self
    }
    /// <p>The name of the global table to describe.</p>
    pub fn set_global_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_table_name(input);
        self
    }
}