// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AutoScalingSettingsDescription {
    /// <p>The minimum capacity units that a global table or global secondary index should be scaled down to.</p>
    #[doc(hidden)]
    pub minimum_units: std::option::Option<i64>,
    /// <p>The maximum capacity units that a global table or global secondary index should be scaled up to.</p>
    #[doc(hidden)]
    pub maximum_units: std::option::Option<i64>,
    /// <p>Disabled auto scaling for this global table or global secondary index.</p>
    #[doc(hidden)]
    pub auto_scaling_disabled: std::option::Option<bool>,
    /// <p>Role ARN used for configuring the auto scaling policy.</p>
    #[doc(hidden)]
    pub auto_scaling_role_arn: std::option::Option<std::string::String>,
    /// <p>Information about the scaling policies.</p>
    #[doc(hidden)]
    pub scaling_policies:
        std::option::Option<std::vec::Vec<crate::types::AutoScalingPolicyDescription>>,
}
impl AutoScalingSettingsDescription {
    /// <p>The minimum capacity units that a global table or global secondary index should be scaled down to.</p>
    pub fn minimum_units(&self) -> std::option::Option<i64> {
        self.minimum_units
    }
    /// <p>The maximum capacity units that a global table or global secondary index should be scaled up to.</p>
    pub fn maximum_units(&self) -> std::option::Option<i64> {
        self.maximum_units
    }
    /// <p>Disabled auto scaling for this global table or global secondary index.</p>
    pub fn auto_scaling_disabled(&self) -> std::option::Option<bool> {
        self.auto_scaling_disabled
    }
    /// <p>Role ARN used for configuring the auto scaling policy.</p>
    pub fn auto_scaling_role_arn(&self) -> std::option::Option<&str> {
        self.auto_scaling_role_arn.as_deref()
    }
    /// <p>Information about the scaling policies.</p>
    pub fn scaling_policies(
        &self,
    ) -> std::option::Option<&[crate::types::AutoScalingPolicyDescription]> {
        self.scaling_policies.as_deref()
    }
}
impl AutoScalingSettingsDescription {
    /// Creates a new builder-style object to manufacture [`AutoScalingSettingsDescription`](crate::types::AutoScalingSettingsDescription).
    pub fn builder() -> crate::types::builders::AutoScalingSettingsDescriptionBuilder {
        crate::types::builders::AutoScalingSettingsDescriptionBuilder::default()
    }
}

/// A builder for [`AutoScalingSettingsDescription`](crate::types::AutoScalingSettingsDescription).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct AutoScalingSettingsDescriptionBuilder {
    pub(crate) minimum_units: std::option::Option<i64>,
    pub(crate) maximum_units: std::option::Option<i64>,
    pub(crate) auto_scaling_disabled: std::option::Option<bool>,
    pub(crate) auto_scaling_role_arn: std::option::Option<std::string::String>,
    pub(crate) scaling_policies:
        std::option::Option<std::vec::Vec<crate::types::AutoScalingPolicyDescription>>,
}
impl AutoScalingSettingsDescriptionBuilder {
    /// <p>The minimum capacity units that a global table or global secondary index should be scaled down to.</p>
    pub fn minimum_units(mut self, input: i64) -> Self {
        self.minimum_units = Some(input);
        self
    }
    /// <p>The minimum capacity units that a global table or global secondary index should be scaled down to.</p>
    pub fn set_minimum_units(mut self, input: std::option::Option<i64>) -> Self {
        self.minimum_units = input;
        self
    }
    /// <p>The maximum capacity units that a global table or global secondary index should be scaled up to.</p>
    pub fn maximum_units(mut self, input: i64) -> Self {
        self.maximum_units = Some(input);
        self
    }
    /// <p>The maximum capacity units that a global table or global secondary index should be scaled up to.</p>
    pub fn set_maximum_units(mut self, input: std::option::Option<i64>) -> Self {
        self.maximum_units = input;
        self
    }
    /// <p>Disabled auto scaling for this global table or global secondary index.</p>
    pub fn auto_scaling_disabled(mut self, input: bool) -> Self {
        self.auto_scaling_disabled = Some(input);
        self
    }
    /// <p>Disabled auto scaling for this global table or global secondary index.</p>
    pub fn set_auto_scaling_disabled(mut self, input: std::option::Option<bool>) -> Self {
        self.auto_scaling_disabled = input;
        self
    }
    /// <p>Role ARN used for configuring the auto scaling policy.</p>
    pub fn auto_scaling_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.auto_scaling_role_arn = Some(input.into());
        self
    }
    /// <p>Role ARN used for configuring the auto scaling policy.</p>
    pub fn set_auto_scaling_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.auto_scaling_role_arn = input;
        self
    }
    /// Appends an item to `scaling_policies`.
    ///
    /// To override the contents of this collection use [`set_scaling_policies`](Self::set_scaling_policies).
    ///
    /// <p>Information about the scaling policies.</p>
    pub fn scaling_policies(mut self, input: crate::types::AutoScalingPolicyDescription) -> Self {
        let mut v = self.scaling_policies.unwrap_or_default();
        v.push(input);
        self.scaling_policies = Some(v);
        self
    }
    /// <p>Information about the scaling policies.</p>
    pub fn set_scaling_policies(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AutoScalingPolicyDescription>>,
    ) -> Self {
        self.scaling_policies = input;
        self
    }
    /// Consumes the builder and constructs a [`AutoScalingSettingsDescription`](crate::types::AutoScalingSettingsDescription).
    pub fn build(self) -> crate::types::AutoScalingSettingsDescription {
        crate::types::AutoScalingSettingsDescription {
            minimum_units: self.minimum_units,
            maximum_units: self.maximum_units,
            auto_scaling_disabled: self.auto_scaling_disabled,
            auto_scaling_role_arn: self.auto_scaling_role_arn,
            scaling_policies: self.scaling_policies,
        }
    }
}
