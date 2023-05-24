// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListImportsOutput {
    /// <p> A list of <code>ImportSummary</code> objects. </p>
    #[doc(hidden)]
    pub import_summary_list: std::option::Option<std::vec::Vec<crate::types::ImportSummary>>,
    /// <p> If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListImports</code> again, with <code>NextToken</code> set to this value. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListImportsOutput {
    /// <p> A list of <code>ImportSummary</code> objects. </p>
    pub fn import_summary_list(&self) -> std::option::Option<&[crate::types::ImportSummary]> {
        self.import_summary_list.as_deref()
    }
    /// <p> If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListImports</code> again, with <code>NextToken</code> set to this value. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListImportsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListImportsOutput {
    /// Creates a new builder-style object to manufacture [`ListImportsOutput`](crate::operation::list_imports::ListImportsOutput).
    pub fn builder() -> crate::operation::list_imports::builders::ListImportsOutputBuilder {
        crate::operation::list_imports::builders::ListImportsOutputBuilder::default()
    }
}

/// A builder for [`ListImportsOutput`](crate::operation::list_imports::ListImportsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListImportsOutputBuilder {
    pub(crate) import_summary_list: std::option::Option<std::vec::Vec<crate::types::ImportSummary>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListImportsOutputBuilder {
    /// Appends an item to `import_summary_list`.
    ///
    /// To override the contents of this collection use [`set_import_summary_list`](Self::set_import_summary_list).
    ///
    /// <p> A list of <code>ImportSummary</code> objects. </p>
    pub fn import_summary_list(mut self, input: crate::types::ImportSummary) -> Self {
        let mut v = self.import_summary_list.unwrap_or_default();
        v.push(input);
        self.import_summary_list = Some(v);
        self
    }
    /// <p> A list of <code>ImportSummary</code> objects. </p>
    pub fn set_import_summary_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ImportSummary>>,
    ) -> Self {
        self.import_summary_list = input;
        self
    }
    /// <p> If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListImports</code> again, with <code>NextToken</code> set to this value. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p> If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListImports</code> again, with <code>NextToken</code> set to this value. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListImportsOutput`](crate::operation::list_imports::ListImportsOutput).
    pub fn build(self) -> crate::operation::list_imports::ListImportsOutput {
        crate::operation::list_imports::ListImportsOutput {
            import_summary_list: self.import_summary_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
