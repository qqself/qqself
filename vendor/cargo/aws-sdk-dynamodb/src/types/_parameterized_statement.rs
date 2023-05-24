// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Represents a PartiQL statment that uses parameters. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ParameterizedStatement {
    /// <p> A PartiQL statment that uses parameters. </p>
    #[doc(hidden)]
    pub statement: std::option::Option<std::string::String>,
    /// <p> The parameter values. </p>
    #[doc(hidden)]
    pub parameters: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
}
impl ParameterizedStatement {
    /// <p> A PartiQL statment that uses parameters. </p>
    pub fn statement(&self) -> std::option::Option<&str> {
        self.statement.as_deref()
    }
    /// <p> The parameter values. </p>
    pub fn parameters(&self) -> std::option::Option<&[crate::types::AttributeValue]> {
        self.parameters.as_deref()
    }
}
impl ParameterizedStatement {
    /// Creates a new builder-style object to manufacture [`ParameterizedStatement`](crate::types::ParameterizedStatement).
    pub fn builder() -> crate::types::builders::ParameterizedStatementBuilder {
        crate::types::builders::ParameterizedStatementBuilder::default()
    }
}

/// A builder for [`ParameterizedStatement`](crate::types::ParameterizedStatement).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ParameterizedStatementBuilder {
    pub(crate) statement: std::option::Option<std::string::String>,
    pub(crate) parameters: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
}
impl ParameterizedStatementBuilder {
    /// <p> A PartiQL statment that uses parameters. </p>
    pub fn statement(mut self, input: impl Into<std::string::String>) -> Self {
        self.statement = Some(input.into());
        self
    }
    /// <p> A PartiQL statment that uses parameters. </p>
    pub fn set_statement(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.statement = input;
        self
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p> The parameter values. </p>
    pub fn parameters(mut self, input: crate::types::AttributeValue) -> Self {
        let mut v = self.parameters.unwrap_or_default();
        v.push(input);
        self.parameters = Some(v);
        self
    }
    /// <p> The parameter values. </p>
    pub fn set_parameters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    ) -> Self {
        self.parameters = input;
        self
    }
    /// Consumes the builder and constructs a [`ParameterizedStatement`](crate::types::ParameterizedStatement).
    pub fn build(self) -> crate::types::ParameterizedStatement {
        crate::types::ParameterizedStatement {
            statement: self.statement,
            parameters: self.parameters,
        }
    }
}
