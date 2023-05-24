// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>DescribeTable</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeTableInput {
    /// <p>The name of the table to describe.</p>
    #[doc(hidden)]
    pub table_name: std::option::Option<std::string::String>,
}
impl DescribeTableInput {
    /// <p>The name of the table to describe.</p>
    pub fn table_name(&self) -> std::option::Option<&str> {
        self.table_name.as_deref()
    }
}
impl DescribeTableInput {
    /// Creates a new builder-style object to manufacture [`DescribeTableInput`](crate::operation::describe_table::DescribeTableInput).
    pub fn builder() -> crate::operation::describe_table::builders::DescribeTableInputBuilder {
        crate::operation::describe_table::builders::DescribeTableInputBuilder::default()
    }
}

/// A builder for [`DescribeTableInput`](crate::operation::describe_table::DescribeTableInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DescribeTableInputBuilder {
    pub(crate) table_name: std::option::Option<std::string::String>,
}
impl DescribeTableInputBuilder {
    /// <p>The name of the table to describe.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.table_name = Some(input.into());
        self
    }
    /// <p>The name of the table to describe.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeTableInput`](crate::operation::describe_table::DescribeTableInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_table::DescribeTableInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::describe_table::DescribeTableInput {
            table_name: self.table_name,
        })
    }
}
