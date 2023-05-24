// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A PartiQL batch statement request. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchStatementRequest {
    /// <p> A valid PartiQL statement. </p>
    #[doc(hidden)]
    pub statement: std::option::Option<std::string::String>,
    /// <p> The parameters associated with a PartiQL statement in the batch request. </p>
    #[doc(hidden)]
    pub parameters: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    /// <p> The read consistency of the PartiQL batch request. </p>
    #[doc(hidden)]
    pub consistent_read: std::option::Option<bool>,
}
impl BatchStatementRequest {
    /// <p> A valid PartiQL statement. </p>
    pub fn statement(&self) -> std::option::Option<&str> {
        self.statement.as_deref()
    }
    /// <p> The parameters associated with a PartiQL statement in the batch request. </p>
    pub fn parameters(&self) -> std::option::Option<&[crate::types::AttributeValue]> {
        self.parameters.as_deref()
    }
    /// <p> The read consistency of the PartiQL batch request. </p>
    pub fn consistent_read(&self) -> std::option::Option<bool> {
        self.consistent_read
    }
}
impl BatchStatementRequest {
    /// Creates a new builder-style object to manufacture [`BatchStatementRequest`](crate::types::BatchStatementRequest).
    pub fn builder() -> crate::types::builders::BatchStatementRequestBuilder {
        crate::types::builders::BatchStatementRequestBuilder::default()
    }
}

/// A builder for [`BatchStatementRequest`](crate::types::BatchStatementRequest).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct BatchStatementRequestBuilder {
    pub(crate) statement: std::option::Option<std::string::String>,
    pub(crate) parameters: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    pub(crate) consistent_read: std::option::Option<bool>,
}
impl BatchStatementRequestBuilder {
    /// <p> A valid PartiQL statement. </p>
    pub fn statement(mut self, input: impl Into<std::string::String>) -> Self {
        self.statement = Some(input.into());
        self
    }
    /// <p> A valid PartiQL statement. </p>
    pub fn set_statement(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.statement = input;
        self
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p> The parameters associated with a PartiQL statement in the batch request. </p>
    pub fn parameters(mut self, input: crate::types::AttributeValue) -> Self {
        let mut v = self.parameters.unwrap_or_default();
        v.push(input);
        self.parameters = Some(v);
        self
    }
    /// <p> The parameters associated with a PartiQL statement in the batch request. </p>
    pub fn set_parameters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    ) -> Self {
        self.parameters = input;
        self
    }
    /// <p> The read consistency of the PartiQL batch request. </p>
    pub fn consistent_read(mut self, input: bool) -> Self {
        self.consistent_read = Some(input);
        self
    }
    /// <p> The read consistency of the PartiQL batch request. </p>
    pub fn set_consistent_read(mut self, input: std::option::Option<bool>) -> Self {
        self.consistent_read = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchStatementRequest`](crate::types::BatchStatementRequest).
    pub fn build(self) -> crate::types::BatchStatementRequest {
        crate::types::BatchStatementRequest {
            statement: self.statement,
            parameters: self.parameters,
            consistent_read: self.consistent_read,
        }
    }
}
