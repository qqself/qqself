// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeBackupInput {
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    #[doc(hidden)]
    pub backup_arn: std::option::Option<std::string::String>,
}
impl DescribeBackupInput {
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn backup_arn(&self) -> std::option::Option<&str> {
        self.backup_arn.as_deref()
    }
}
impl DescribeBackupInput {
    /// Creates a new builder-style object to manufacture [`DescribeBackupInput`](crate::operation::describe_backup::DescribeBackupInput).
    pub fn builder() -> crate::operation::describe_backup::builders::DescribeBackupInputBuilder {
        crate::operation::describe_backup::builders::DescribeBackupInputBuilder::default()
    }
}

/// A builder for [`DescribeBackupInput`](crate::operation::describe_backup::DescribeBackupInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DescribeBackupInputBuilder {
    pub(crate) backup_arn: std::option::Option<std::string::String>,
}
impl DescribeBackupInputBuilder {
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn backup_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.backup_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn set_backup_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.backup_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeBackupInput`](crate::operation::describe_backup::DescribeBackupInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_backup::DescribeBackupInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::describe_backup::DescribeBackupInput {
            backup_arn: self.backup_arn,
        })
    }
}
