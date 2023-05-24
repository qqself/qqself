// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The description of the point in time settings applied to the table.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PointInTimeRecoveryDescription {
    /// <p>The current state of point in time recovery:</p>
    /// <ul>
    /// <li> <p> <code>ENABLED</code> - Point in time recovery is enabled.</p> </li>
    /// <li> <p> <code>DISABLED</code> - Point in time recovery is disabled.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub point_in_time_recovery_status: std::option::Option<crate::types::PointInTimeRecoveryStatus>,
    /// <p>Specifies the earliest point in time you can restore your table to. You can restore your table to any point in time during the last 35 days. </p>
    #[doc(hidden)]
    pub earliest_restorable_date_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. </p>
    #[doc(hidden)]
    pub latest_restorable_date_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl PointInTimeRecoveryDescription {
    /// <p>The current state of point in time recovery:</p>
    /// <ul>
    /// <li> <p> <code>ENABLED</code> - Point in time recovery is enabled.</p> </li>
    /// <li> <p> <code>DISABLED</code> - Point in time recovery is disabled.</p> </li>
    /// </ul>
    pub fn point_in_time_recovery_status(
        &self,
    ) -> std::option::Option<&crate::types::PointInTimeRecoveryStatus> {
        self.point_in_time_recovery_status.as_ref()
    }
    /// <p>Specifies the earliest point in time you can restore your table to. You can restore your table to any point in time during the last 35 days. </p>
    pub fn earliest_restorable_date_time(
        &self,
    ) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.earliest_restorable_date_time.as_ref()
    }
    /// <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. </p>
    pub fn latest_restorable_date_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.latest_restorable_date_time.as_ref()
    }
}
impl PointInTimeRecoveryDescription {
    /// Creates a new builder-style object to manufacture [`PointInTimeRecoveryDescription`](crate::types::PointInTimeRecoveryDescription).
    pub fn builder() -> crate::types::builders::PointInTimeRecoveryDescriptionBuilder {
        crate::types::builders::PointInTimeRecoveryDescriptionBuilder::default()
    }
}

/// A builder for [`PointInTimeRecoveryDescription`](crate::types::PointInTimeRecoveryDescription).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct PointInTimeRecoveryDescriptionBuilder {
    pub(crate) point_in_time_recovery_status:
        std::option::Option<crate::types::PointInTimeRecoveryStatus>,
    pub(crate) earliest_restorable_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) latest_restorable_date_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl PointInTimeRecoveryDescriptionBuilder {
    /// <p>The current state of point in time recovery:</p>
    /// <ul>
    /// <li> <p> <code>ENABLED</code> - Point in time recovery is enabled.</p> </li>
    /// <li> <p> <code>DISABLED</code> - Point in time recovery is disabled.</p> </li>
    /// </ul>
    pub fn point_in_time_recovery_status(
        mut self,
        input: crate::types::PointInTimeRecoveryStatus,
    ) -> Self {
        self.point_in_time_recovery_status = Some(input);
        self
    }
    /// <p>The current state of point in time recovery:</p>
    /// <ul>
    /// <li> <p> <code>ENABLED</code> - Point in time recovery is enabled.</p> </li>
    /// <li> <p> <code>DISABLED</code> - Point in time recovery is disabled.</p> </li>
    /// </ul>
    pub fn set_point_in_time_recovery_status(
        mut self,
        input: std::option::Option<crate::types::PointInTimeRecoveryStatus>,
    ) -> Self {
        self.point_in_time_recovery_status = input;
        self
    }
    /// <p>Specifies the earliest point in time you can restore your table to. You can restore your table to any point in time during the last 35 days. </p>
    pub fn earliest_restorable_date_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.earliest_restorable_date_time = Some(input);
        self
    }
    /// <p>Specifies the earliest point in time you can restore your table to. You can restore your table to any point in time during the last 35 days. </p>
    pub fn set_earliest_restorable_date_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.earliest_restorable_date_time = input;
        self
    }
    /// <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. </p>
    pub fn latest_restorable_date_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.latest_restorable_date_time = Some(input);
        self
    }
    /// <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. </p>
    pub fn set_latest_restorable_date_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.latest_restorable_date_time = input;
        self
    }
    /// Consumes the builder and constructs a [`PointInTimeRecoveryDescription`](crate::types::PointInTimeRecoveryDescription).
    pub fn build(self) -> crate::types::PointInTimeRecoveryDescription {
        crate::types::PointInTimeRecoveryDescription {
            point_in_time_recovery_status: self.point_in_time_recovery_status,
            earliest_restorable_date_time: self.earliest_restorable_date_time,
            latest_restorable_date_time: self.latest_restorable_date_time,
        }
    }
}