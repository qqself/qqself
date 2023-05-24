// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>DeleteTable</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteTableInput {
    /// <p>The name of the table to delete.</p>
    #[doc(hidden)]
    pub table_name: std::option::Option<std::string::String>,
}
impl DeleteTableInput {
    /// <p>The name of the table to delete.</p>
    pub fn table_name(&self) -> std::option::Option<&str> {
        self.table_name.as_deref()
    }
}
impl DeleteTableInput {
    /// Creates a new builder-style object to manufacture [`DeleteTableInput`](crate::operation::delete_table::DeleteTableInput).
    pub fn builder() -> crate::operation::delete_table::builders::DeleteTableInputBuilder {
        crate::operation::delete_table::builders::DeleteTableInputBuilder::default()
    }
}

/// A builder for [`DeleteTableInput`](crate::operation::delete_table::DeleteTableInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DeleteTableInputBuilder {
    pub(crate) table_name: std::option::Option<std::string::String>,
}
impl DeleteTableInputBuilder {
    /// <p>The name of the table to delete.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.table_name = Some(input.into());
        self
    }
    /// <p>The name of the table to delete.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTableInput`](crate::operation::delete_table::DeleteTableInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_table::DeleteTableInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_table::DeleteTableInput {
            table_name: self.table_name,
        })
    }
}
