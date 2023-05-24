// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListBackupsOutput {
    /// <p>List of <code>BackupSummary</code> objects.</p>
    #[doc(hidden)]
    pub backup_summaries: std::option::Option<std::vec::Vec<crate::types::BackupSummary>>,
    /// <p> The ARN of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is empty, then the last page of results has been processed and there are no more results to be retrieved. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is not empty, this may or may not indicate that there is more data to be returned. All results are guaranteed to have been returned if and only if no value for <code>LastEvaluatedBackupArn</code> is returned. </p>
    #[doc(hidden)]
    pub last_evaluated_backup_arn: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListBackupsOutput {
    /// <p>List of <code>BackupSummary</code> objects.</p>
    pub fn backup_summaries(&self) -> std::option::Option<&[crate::types::BackupSummary]> {
        self.backup_summaries.as_deref()
    }
    /// <p> The ARN of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is empty, then the last page of results has been processed and there are no more results to be retrieved. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is not empty, this may or may not indicate that there is more data to be returned. All results are guaranteed to have been returned if and only if no value for <code>LastEvaluatedBackupArn</code> is returned. </p>
    pub fn last_evaluated_backup_arn(&self) -> std::option::Option<&str> {
        self.last_evaluated_backup_arn.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListBackupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListBackupsOutput {
    /// Creates a new builder-style object to manufacture [`ListBackupsOutput`](crate::operation::list_backups::ListBackupsOutput).
    pub fn builder() -> crate::operation::list_backups::builders::ListBackupsOutputBuilder {
        crate::operation::list_backups::builders::ListBackupsOutputBuilder::default()
    }
}

/// A builder for [`ListBackupsOutput`](crate::operation::list_backups::ListBackupsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListBackupsOutputBuilder {
    pub(crate) backup_summaries: std::option::Option<std::vec::Vec<crate::types::BackupSummary>>,
    pub(crate) last_evaluated_backup_arn: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListBackupsOutputBuilder {
    /// Appends an item to `backup_summaries`.
    ///
    /// To override the contents of this collection use [`set_backup_summaries`](Self::set_backup_summaries).
    ///
    /// <p>List of <code>BackupSummary</code> objects.</p>
    pub fn backup_summaries(mut self, input: crate::types::BackupSummary) -> Self {
        let mut v = self.backup_summaries.unwrap_or_default();
        v.push(input);
        self.backup_summaries = Some(v);
        self
    }
    /// <p>List of <code>BackupSummary</code> objects.</p>
    pub fn set_backup_summaries(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::BackupSummary>>,
    ) -> Self {
        self.backup_summaries = input;
        self
    }
    /// <p> The ARN of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is empty, then the last page of results has been processed and there are no more results to be retrieved. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is not empty, this may or may not indicate that there is more data to be returned. All results are guaranteed to have been returned if and only if no value for <code>LastEvaluatedBackupArn</code> is returned. </p>
    pub fn last_evaluated_backup_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.last_evaluated_backup_arn = Some(input.into());
        self
    }
    /// <p> The ARN of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is empty, then the last page of results has been processed and there are no more results to be retrieved. </p>
    /// <p> If <code>LastEvaluatedBackupArn</code> is not empty, this may or may not indicate that there is more data to be returned. All results are guaranteed to have been returned if and only if no value for <code>LastEvaluatedBackupArn</code> is returned. </p>
    pub fn set_last_evaluated_backup_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.last_evaluated_backup_arn = input;
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
    /// Consumes the builder and constructs a [`ListBackupsOutput`](crate::operation::list_backups::ListBackupsOutput).
    pub fn build(self) -> crate::operation::list_backups::ListBackupsOutput {
        crate::operation::list_backups::ListBackupsOutput {
            backup_summaries: self.backup_summaries,
            last_evaluated_backup_arn: self.last_evaluated_backup_arn,
            _request_id: self._request_id,
        }
    }
}
