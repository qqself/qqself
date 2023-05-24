// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary information about an export task.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ExportSummary {
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    #[doc(hidden)]
    pub export_arn: std::option::Option<std::string::String>,
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    #[doc(hidden)]
    pub export_status: std::option::Option<crate::types::ExportStatus>,
}
impl ExportSummary {
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn export_arn(&self) -> std::option::Option<&str> {
        self.export_arn.as_deref()
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn export_status(&self) -> std::option::Option<&crate::types::ExportStatus> {
        self.export_status.as_ref()
    }
}
impl ExportSummary {
    /// Creates a new builder-style object to manufacture [`ExportSummary`](crate::types::ExportSummary).
    pub fn builder() -> crate::types::builders::ExportSummaryBuilder {
        crate::types::builders::ExportSummaryBuilder::default()
    }
}

/// A builder for [`ExportSummary`](crate::types::ExportSummary).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ExportSummaryBuilder {
    pub(crate) export_arn: std::option::Option<std::string::String>,
    pub(crate) export_status: std::option::Option<crate::types::ExportStatus>,
}
impl ExportSummaryBuilder {
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn export_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.export_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn set_export_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.export_arn = input;
        self
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn export_status(mut self, input: crate::types::ExportStatus) -> Self {
        self.export_status = Some(input);
        self
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn set_export_status(
        mut self,
        input: std::option::Option<crate::types::ExportStatus>,
    ) -> Self {
        self.export_status = input;
        self
    }
    /// Consumes the builder and constructs a [`ExportSummary`](crate::types::ExportSummary).
    pub fn build(self) -> crate::types::ExportSummary {
        crate::types::ExportSummary {
            export_arn: self.export_arn,
            export_status: self.export_status,
        }
    }
}
