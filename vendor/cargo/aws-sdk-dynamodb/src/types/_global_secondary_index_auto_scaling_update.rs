// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the auto scaling settings of a global secondary index for a global table that will be modified.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GlobalSecondaryIndexAutoScalingUpdate {
    /// <p>The name of the global secondary index.</p>
    #[doc(hidden)]
    pub index_name: std::option::Option<std::string::String>,
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    #[doc(hidden)]
    pub provisioned_write_capacity_auto_scaling_update:
        std::option::Option<crate::types::AutoScalingSettingsUpdate>,
}
impl GlobalSecondaryIndexAutoScalingUpdate {
    /// <p>The name of the global secondary index.</p>
    pub fn index_name(&self) -> std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    pub fn provisioned_write_capacity_auto_scaling_update(
        &self,
    ) -> std::option::Option<&crate::types::AutoScalingSettingsUpdate> {
        self.provisioned_write_capacity_auto_scaling_update.as_ref()
    }
}
impl GlobalSecondaryIndexAutoScalingUpdate {
    /// Creates a new builder-style object to manufacture [`GlobalSecondaryIndexAutoScalingUpdate`](crate::types::GlobalSecondaryIndexAutoScalingUpdate).
    pub fn builder() -> crate::types::builders::GlobalSecondaryIndexAutoScalingUpdateBuilder {
        crate::types::builders::GlobalSecondaryIndexAutoScalingUpdateBuilder::default()
    }
}

/// A builder for [`GlobalSecondaryIndexAutoScalingUpdate`](crate::types::GlobalSecondaryIndexAutoScalingUpdate).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct GlobalSecondaryIndexAutoScalingUpdateBuilder {
    pub(crate) index_name: std::option::Option<std::string::String>,
    pub(crate) provisioned_write_capacity_auto_scaling_update:
        std::option::Option<crate::types::AutoScalingSettingsUpdate>,
}
impl GlobalSecondaryIndexAutoScalingUpdateBuilder {
    /// <p>The name of the global secondary index.</p>
    pub fn index_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.index_name = Some(input.into());
        self
    }
    /// <p>The name of the global secondary index.</p>
    pub fn set_index_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    pub fn provisioned_write_capacity_auto_scaling_update(
        mut self,
        input: crate::types::AutoScalingSettingsUpdate,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_update = Some(input);
        self
    }
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    pub fn set_provisioned_write_capacity_auto_scaling_update(
        mut self,
        input: std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_update = input;
        self
    }
    /// Consumes the builder and constructs a [`GlobalSecondaryIndexAutoScalingUpdate`](crate::types::GlobalSecondaryIndexAutoScalingUpdate).
    pub fn build(self) -> crate::types::GlobalSecondaryIndexAutoScalingUpdate {
        crate::types::GlobalSecondaryIndexAutoScalingUpdate {
            index_name: self.index_name,
            provisioned_write_capacity_auto_scaling_update: self
                .provisioned_write_capacity_auto_scaling_update,
        }
    }
}
