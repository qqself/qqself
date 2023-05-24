// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The identifiers for the temporary security credentials that the operation returns.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AssumedRoleUser {
    /// <p>A unique identifier that contains the role ID and the role session name of the role that is being assumed. The role ID is generated by Amazon Web Services when the role is created.</p>
    #[doc(hidden)]
    pub assumed_role_id: std::option::Option<std::string::String>,
    /// <p>The ARN of the temporary security credentials that are returned from the <code>AssumeRole</code> action. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
}
impl AssumedRoleUser {
    /// <p>A unique identifier that contains the role ID and the role session name of the role that is being assumed. The role ID is generated by Amazon Web Services when the role is created.</p>
    pub fn assumed_role_id(&self) -> std::option::Option<&str> {
        self.assumed_role_id.as_deref()
    }
    /// <p>The ARN of the temporary security credentials that are returned from the <code>AssumeRole</code> action. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl AssumedRoleUser {
    /// Creates a new builder-style object to manufacture [`AssumedRoleUser`](crate::types::AssumedRoleUser).
    pub fn builder() -> crate::types::builders::AssumedRoleUserBuilder {
        crate::types::builders::AssumedRoleUserBuilder::default()
    }
}

/// A builder for [`AssumedRoleUser`](crate::types::AssumedRoleUser).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct AssumedRoleUserBuilder {
    pub(crate) assumed_role_id: std::option::Option<std::string::String>,
    pub(crate) arn: std::option::Option<std::string::String>,
}
impl AssumedRoleUserBuilder {
    /// <p>A unique identifier that contains the role ID and the role session name of the role that is being assumed. The role ID is generated by Amazon Web Services when the role is created.</p>
    pub fn assumed_role_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.assumed_role_id = Some(input.into());
        self
    }
    /// <p>A unique identifier that contains the role ID and the role session name of the role that is being assumed. The role ID is generated by Amazon Web Services when the role is created.</p>
    pub fn set_assumed_role_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.assumed_role_id = input;
        self
    }
    /// <p>The ARN of the temporary security credentials that are returned from the <code>AssumeRole</code> action. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The ARN of the temporary security credentials that are returned from the <code>AssumeRole</code> action. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`AssumedRoleUser`](crate::types::AssumedRoleUser).
    pub fn build(self) -> crate::types::AssumedRoleUser {
        crate::types::AssumedRoleUser {
            assumed_role_id: self.assumed_role_id,
            arn: self.arn,
        }
    }
}
