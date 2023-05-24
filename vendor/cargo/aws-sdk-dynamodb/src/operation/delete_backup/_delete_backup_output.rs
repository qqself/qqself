// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteBackupOutput {
    /// <p>Contains the description of the backup created for the table.</p>
    #[doc(hidden)]
    pub backup_description: std::option::Option<crate::types::BackupDescription>,
    _request_id: Option<String>,
}
impl DeleteBackupOutput {
    /// <p>Contains the description of the backup created for the table.</p>
    pub fn backup_description(&self) -> std::option::Option<&crate::types::BackupDescription> {
        self.backup_description.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteBackupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteBackupOutput {
    /// Creates a new builder-style object to manufacture [`DeleteBackupOutput`](crate::operation::delete_backup::DeleteBackupOutput).
    pub fn builder() -> crate::operation::delete_backup::builders::DeleteBackupOutputBuilder {
        crate::operation::delete_backup::builders::DeleteBackupOutputBuilder::default()
    }
}

/// A builder for [`DeleteBackupOutput`](crate::operation::delete_backup::DeleteBackupOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DeleteBackupOutputBuilder {
    pub(crate) backup_description: std::option::Option<crate::types::BackupDescription>,
    _request_id: Option<String>,
}
impl DeleteBackupOutputBuilder {
    /// <p>Contains the description of the backup created for the table.</p>
    pub fn backup_description(mut self, input: crate::types::BackupDescription) -> Self {
        self.backup_description = Some(input);
        self
    }
    /// <p>Contains the description of the backup created for the table.</p>
    pub fn set_backup_description(
        mut self,
        input: std::option::Option<crate::types::BackupDescription>,
    ) -> Self {
        self.backup_description = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteBackupOutput`](crate::operation::delete_backup::DeleteBackupOutput).
    pub fn build(self) -> crate::operation::delete_backup::DeleteBackupOutput {
        crate::operation::delete_backup::DeleteBackupOutput {
            backup_description: self.backup_description,
            _request_id: self._request_id,
        }
    }
}
