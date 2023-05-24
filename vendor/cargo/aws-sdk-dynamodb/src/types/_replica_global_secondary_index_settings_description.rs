// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the properties of a global secondary index.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ReplicaGlobalSecondaryIndexSettingsDescription {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    #[doc(hidden)]
    pub index_name: std::option::Option<std::string::String>,
    /// <p> The current status of the global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The global secondary index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The global secondary index is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The global secondary index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The global secondary index is ready for use.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub index_status: std::option::Option<crate::types::IndexStatus>,
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    #[doc(hidden)]
    pub provisioned_read_capacity_units: std::option::Option<i64>,
    /// <p>Auto scaling settings for a global secondary index replica's read capacity units.</p>
    #[doc(hidden)]
    pub provisioned_read_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    #[doc(hidden)]
    pub provisioned_write_capacity_units: std::option::Option<i64>,
    /// <p>Auto scaling settings for a global secondary index replica's write capacity units.</p>
    #[doc(hidden)]
    pub provisioned_write_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
}
impl ReplicaGlobalSecondaryIndexSettingsDescription {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub fn index_name(&self) -> std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p> The current status of the global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The global secondary index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The global secondary index is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The global secondary index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The global secondary index is ready for use.</p> </li>
    /// </ul>
    pub fn index_status(&self) -> std::option::Option<&crate::types::IndexStatus> {
        self.index_status.as_ref()
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    pub fn provisioned_read_capacity_units(&self) -> std::option::Option<i64> {
        self.provisioned_read_capacity_units
    }
    /// <p>Auto scaling settings for a global secondary index replica's read capacity units.</p>
    pub fn provisioned_read_capacity_auto_scaling_settings(
        &self,
    ) -> std::option::Option<&crate::types::AutoScalingSettingsDescription> {
        self.provisioned_read_capacity_auto_scaling_settings
            .as_ref()
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    pub fn provisioned_write_capacity_units(&self) -> std::option::Option<i64> {
        self.provisioned_write_capacity_units
    }
    /// <p>Auto scaling settings for a global secondary index replica's write capacity units.</p>
    pub fn provisioned_write_capacity_auto_scaling_settings(
        &self,
    ) -> std::option::Option<&crate::types::AutoScalingSettingsDescription> {
        self.provisioned_write_capacity_auto_scaling_settings
            .as_ref()
    }
}
impl ReplicaGlobalSecondaryIndexSettingsDescription {
    /// Creates a new builder-style object to manufacture [`ReplicaGlobalSecondaryIndexSettingsDescription`](crate::types::ReplicaGlobalSecondaryIndexSettingsDescription).
    pub fn builder() -> crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder
    {
        crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder::default()
    }
}

/// A builder for [`ReplicaGlobalSecondaryIndexSettingsDescription`](crate::types::ReplicaGlobalSecondaryIndexSettingsDescription).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder {
    pub(crate) index_name: std::option::Option<std::string::String>,
    pub(crate) index_status: std::option::Option<crate::types::IndexStatus>,
    pub(crate) provisioned_read_capacity_units: std::option::Option<i64>,
    pub(crate) provisioned_read_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
    pub(crate) provisioned_write_capacity_units: std::option::Option<i64>,
    pub(crate) provisioned_write_capacity_auto_scaling_settings:
        std::option::Option<crate::types::AutoScalingSettingsDescription>,
}
impl ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub fn index_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.index_name = Some(input.into());
        self
    }
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    pub fn set_index_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p> The current status of the global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The global secondary index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The global secondary index is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The global secondary index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The global secondary index is ready for use.</p> </li>
    /// </ul>
    pub fn index_status(mut self, input: crate::types::IndexStatus) -> Self {
        self.index_status = Some(input);
        self
    }
    /// <p> The current status of the global secondary index:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The global secondary index is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The global secondary index is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The global secondary index is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The global secondary index is ready for use.</p> </li>
    /// </ul>
    pub fn set_index_status(
        mut self,
        input: std::option::Option<crate::types::IndexStatus>,
    ) -> Self {
        self.index_status = input;
        self
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    pub fn provisioned_read_capacity_units(mut self, input: i64) -> Self {
        self.provisioned_read_capacity_units = Some(input);
        self
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    pub fn set_provisioned_read_capacity_units(mut self, input: std::option::Option<i64>) -> Self {
        self.provisioned_read_capacity_units = input;
        self
    }
    /// <p>Auto scaling settings for a global secondary index replica's read capacity units.</p>
    pub fn provisioned_read_capacity_auto_scaling_settings(
        mut self,
        input: crate::types::AutoScalingSettingsDescription,
    ) -> Self {
        self.provisioned_read_capacity_auto_scaling_settings = Some(input);
        self
    }
    /// <p>Auto scaling settings for a global secondary index replica's read capacity units.</p>
    pub fn set_provisioned_read_capacity_auto_scaling_settings(
        mut self,
        input: std::option::Option<crate::types::AutoScalingSettingsDescription>,
    ) -> Self {
        self.provisioned_read_capacity_auto_scaling_settings = input;
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    pub fn provisioned_write_capacity_units(mut self, input: i64) -> Self {
        self.provisioned_write_capacity_units = Some(input);
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    pub fn set_provisioned_write_capacity_units(mut self, input: std::option::Option<i64>) -> Self {
        self.provisioned_write_capacity_units = input;
        self
    }
    /// <p>Auto scaling settings for a global secondary index replica's write capacity units.</p>
    pub fn provisioned_write_capacity_auto_scaling_settings(
        mut self,
        input: crate::types::AutoScalingSettingsDescription,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_settings = Some(input);
        self
    }
    /// <p>Auto scaling settings for a global secondary index replica's write capacity units.</p>
    pub fn set_provisioned_write_capacity_auto_scaling_settings(
        mut self,
        input: std::option::Option<crate::types::AutoScalingSettingsDescription>,
    ) -> Self {
        self.provisioned_write_capacity_auto_scaling_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicaGlobalSecondaryIndexSettingsDescription`](crate::types::ReplicaGlobalSecondaryIndexSettingsDescription).
    pub fn build(self) -> crate::types::ReplicaGlobalSecondaryIndexSettingsDescription {
        crate::types::ReplicaGlobalSecondaryIndexSettingsDescription {
            index_name: self.index_name,
            index_status: self.index_status,
            provisioned_read_capacity_units: self.provisioned_read_capacity_units,
            provisioned_read_capacity_auto_scaling_settings: self
                .provisioned_read_capacity_auto_scaling_settings,
            provisioned_write_capacity_units: self.provisioned_write_capacity_units,
            provisioned_write_capacity_auto_scaling_settings: self
                .provisioned_write_capacity_auto_scaling_settings,
        }
    }
}
