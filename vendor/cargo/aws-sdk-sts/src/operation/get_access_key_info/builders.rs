// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_access_key_info::_get_access_key_info_output::GetAccessKeyInfoOutputBuilder;

pub use crate::operation::get_access_key_info::_get_access_key_info_input::GetAccessKeyInfoInputBuilder;

/// Fluent builder constructing a request to `GetAccessKeyInfo`.
///
/// <p>Returns the account identifier for the specified access key ID.</p>
/// <p>Access keys consist of two parts: an access key ID (for example, <code>AKIAIOSFODNN7EXAMPLE</code>) and a secret access key (for example, <code>wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY</code>). For more information about access keys, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html">Managing Access Keys for IAM Users</a> in the <i>IAM User Guide</i>.</p>
/// <p>When you pass an access key ID to this operation, it returns the ID of the Amazon Web Services account to which the keys belong. Access key IDs beginning with <code>AKIA</code> are long-term credentials for an IAM user or the Amazon Web Services account root user. Access key IDs beginning with <code>ASIA</code> are temporary credentials that are created using STS operations. If the account in the response belongs to you, you can sign in as the root user and review your root user access keys. Then, you can pull a <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_getting-report.html">credentials report</a> to learn which IAM user owns the keys. To learn who requested the temporary credentials for an <code>ASIA</code> access key, view the STS events in your <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/cloudtrail-integration.html">CloudTrail logs</a> in the <i>IAM User Guide</i>.</p>
/// <p>This operation does not indicate the state of the access key. The key might be active, inactive, or deleted. Active keys might not have permissions to perform an operation. Providing a deleted access key might return an error that the key doesn't exist.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetAccessKeyInfoFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_access_key_info::builders::GetAccessKeyInfoInputBuilder,
}
impl GetAccessKeyInfoFluentBuilder {
    /// Creates a new `GetAccessKeyInfo`.
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
            crate::operation::get_access_key_info::GetAccessKeyInfo,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_access_key_info::GetAccessKeyInfoError,
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
        crate::operation::get_access_key_info::GetAccessKeyInfoOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_access_key_info::GetAccessKeyInfoError,
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
    /// <p>The identifier of an access key.</p>
    /// <p>This parameter allows (through its regex pattern) a string of characters that can consist of any upper- or lowercase letter or digit.</p>
    pub fn access_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.access_key_id(input.into());
        self
    }
    /// <p>The identifier of an access key.</p>
    /// <p>This parameter allows (through its regex pattern) a string of characters that can consist of any upper- or lowercase letter or digit.</p>
    pub fn set_access_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_access_key_id(input);
        self
    }
}