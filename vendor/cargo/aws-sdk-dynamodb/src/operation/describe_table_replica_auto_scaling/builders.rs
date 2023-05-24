// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_output::DescribeTableReplicaAutoScalingOutputBuilder;

pub use crate::operation::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_input::DescribeTableReplicaAutoScalingInputBuilder;

/// Fluent builder constructing a request to `DescribeTableReplicaAutoScaling`.
///
/// <p>Describes auto scaling settings across replicas of the global table at once.</p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> of global tables.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTableReplicaAutoScalingFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingInputBuilder,
}
impl DescribeTableReplicaAutoScalingFluentBuilder {
    /// Creates a new `DescribeTableReplicaAutoScaling`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }
    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScaling, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput, aws_smithy_http::result::SdkError<crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
}
