// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>DynamoDB rejected the request because you retried a request with a different payload but with an idempotent token that was already used.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct IdempotentParameterMismatchException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl IdempotentParameterMismatchException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for IdempotentParameterMismatchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdempotentParameterMismatchException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for IdempotentParameterMismatchException {}
impl aws_http::request_id::RequestId for crate::types::error::IdempotentParameterMismatchException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for IdempotentParameterMismatchException
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl IdempotentParameterMismatchException {
    /// Creates a new builder-style object to manufacture [`IdempotentParameterMismatchException`](crate::types::error::IdempotentParameterMismatchException).
    pub fn builder() -> crate::types::error::builders::IdempotentParameterMismatchExceptionBuilder {
        crate::types::error::builders::IdempotentParameterMismatchExceptionBuilder::default()
    }
}

/// A builder for [`IdempotentParameterMismatchException`](crate::types::error::IdempotentParameterMismatchException).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct IdempotentParameterMismatchExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl IdempotentParameterMismatchExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`IdempotentParameterMismatchException`](crate::types::error::IdempotentParameterMismatchException).
    pub fn build(self) -> crate::types::error::IdempotentParameterMismatchException {
        crate::types::error::IdempotentParameterMismatchException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
