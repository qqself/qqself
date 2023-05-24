// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the auto scaling configuration for a replica global secondary index.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingDescription {
    /// <p>The name of the global secondary index.</p>
    #[doc(hidden)]
    pub index_name: std::option::Option<std::string::String>,
    /// <p>The current state of the replica global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The table/index configuration is being updated. The table/index remains available for data operations when <code>UPDATING</code> </p> </li>
    /// <li> <p> <code>DELETING</code> - The index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The index is ready for use.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub index_status: std::option::Option<crate::types::IndexStatus>,
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    #[doc(hidden)]
    pub provisioned_read_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    #[doc(hidden)]
    pub provisioned_write_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
}
impl ReplicaGlobalSecondaryIndexAutoScalingDescription {
    /// <p>The name of the global secondary index.</p>
    pub fn index_name(&self) -> std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p>The current state of the replica global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The table/index configuration is being updated. The table/index remains available for data operations when <code>UPDATING</code> </p> </li>
    /// <li> <p> <code>DELETING</code> - The index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The index is ready for use.</p> </li>
    /// </ul>
    pub fn index_status(&self) -> std::option::Option<&crate::types::IndexStatus> {
        self.index_status.as_ref()
    }
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    pub fn provisioned_read_capacity_auto_scaling_settings(
        &self,
    ) -> std::option::Option<&crate::types::AutoScalingSettingsDescription> {
        self.provisioned_read_capacity_auto_scaling_settings
            .as_ref()
    }
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    pub fn provisioned_write_capacity_auto_scaling_settings(
        &self,
    ) -> std::option::Option<&crate::types::AutoScalingSettingsDescription> {
        self.provisioned_write_capacity_auto_scaling_settings
            .as_ref()
    }
}
impl ReplicaGlobalSecondaryIndexAutoScalingDescription {
    /// Creates a new builder-style object to manufacture [`ReplicaGlobalSecondaryIndexAutoScalingDescription`](crate::types::ReplicaGlobalSecondaryIndexAutoScalingDescription).
    pub fn builder(
    ) -> crate::types::builders::ReplicaGlobalSecondaryIndexAutoScalingDescriptionBuilder {
        crate::types::builders::ReplicaGlobalSecondaryIndexAutoScalingDescriptionBuilder::default()
    }
}

/// A builder for [`ReplicaGlobalSecondaryIndexAutoScalingDescription`](crate::types::ReplicaGlobalSecondaryIndexAutoScalingDescription).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingDescriptionBuilder {
    pub(crate) index_name: std::option::Option<std::string::String>,
    pub(crate) index_status: std::option::Option<crate::types::IndexStatus>,
    pub(crate) provisioned_read_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
    pub(crate) provisioned_write_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
}
impl ReplicaGlobalSecondaryIndexAutoScalingDescriptionBuilder {
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
    /// <p>The current state of the replica global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The table/index configuration is being updated. The table/index remains available for data operations when <code>UPDATING</code> </p> </li>
    /// <li> <p> <code>DELETING</code> - The index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The index is ready for use.</p> </li>
    /// </ul>
    pub fn index_status(mut self, input: crate::types::IndexStatus) -> Self {
        self.index_status = Some(input);
        self
    }
    /// <p>The current state of the replica global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The table/index configuration is being updated. The table/index remains available for data operations when <code>UPDATING</code> </p> </li>
    /// <li> <p> <code>DELETING</code> - The index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The index is ready for use.</p> </li>
    /// </ul>
    pub fn set_index_status(
        mut self,
        input: std::option::Option<crate::types::IndexStatus>,
    ) -> Self {
        self.index_status = input;
        self
    }
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    pub fn provisioned_read_capacity_auto_scaling_settings(
        mut self,
        input: crate::types::AutoScalingSettingsDescription,
    ) -> Self {
        self.provisioned_read_capacity_auto_scaling_settings = Some(input);
        self
    }
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    pub fn set_provisioned_read_capacity_auto_scaling_settings(
        mut self,
        input: std::option::Option<crate::types::AutoScalingSettingsDescription>,
    ) -> Self {
        self.provisioned_read_capacity_auto_scaling_settings = input;
        self
    }
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    pub fn provisioned_write_capacity_auto_scaling_settings(
        mut self,
        input: crate::types::AutoScalingSettingsDescription,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_settings = Some(input);
        self
    }
    /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
    pub fn set_provisioned_write_capacity_auto_scaling_settings(
        mut self,
        input: std::option::Option<crate::types::AutoScalingSettingsDescription>,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicaGlobalSecondaryIndexAutoScalingDescription`](crate::types::ReplicaGlobalSecondaryIndexAutoScalingDescription).
    pub fn build(self) -> crate::types::ReplicaGlobalSecondaryIndexAutoScalingDescription {
        crate::types::ReplicaGlobalSecondaryIndexAutoScalingDescription {
            index_name: self.index_name,
            index_status: self.index_status,
            provisioned_read_capacity_auto_scaling_settings: self
                .provisioned_read_capacity_auto_scaling_settings,
            provisioned_write_capacity_auto_scaling_settings: self
                .provisioned_write_capacity_auto_scaling_settings,
        }
    }
}