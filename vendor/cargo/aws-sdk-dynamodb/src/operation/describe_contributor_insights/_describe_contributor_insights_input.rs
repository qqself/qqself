// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeContributorInsightsInput {
    /// <p>The name of the table to describe.</p>
    #[doc(hidden)]
    pub table_name: std::option::Option<std::string::String>,
    /// <p>The name of the global secondary index to describe, if applicable.</p>
    #[doc(hidden)]
    pub index_name: std::option::Option<std::string::String>,
}
impl DescribeContributorInsightsInput {
    /// <p>The name of the table to describe.</p>
    pub fn table_name(&self) -> std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>The name of the global secondary index to describe, if applicable.</p>
    pub fn index_name(&self) -> std::option::Option<&str> {
        self.index_name.as_deref()
    }
}
impl DescribeContributorInsightsInput {
    /// Creates a new builder-style object to manufacture [`DescribeContributorInsightsInput`](crate::operation::describe_contributor_insights::DescribeContributorInsightsInput).
    pub fn builder() -> crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsInputBuilder{
        crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsInputBuilder::default()
    }
}

/// A builder for [`DescribeContributorInsightsInput`](crate::operation::describe_contributor_insights::DescribeContributorInsightsInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DescribeContributorInsightsInputBuilder {
    pub(crate) table_name: std::option::Option<std::string::String>,
    pub(crate) index_name: std::option::Option<std::string::String>,
}
impl DescribeContributorInsightsInputBuilder {
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
    /// <p>The name of the global secondary index to describe, if applicable.</p>
    pub fn index_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.index_name = Some(input.into());
        self
    }
    /// <p>The name of the global secondary index to describe, if applicable.</p>
    pub fn set_index_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeContributorInsightsInput`](crate::operation::describe_contributor_insights::DescribeContributorInsightsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_contributor_insights::DescribeContributorInsightsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_contributor_insights::DescribeContributorInsightsInput {
                table_name: self.table_name,
                index_name: self.index_name,
            },
        )
    }
}
