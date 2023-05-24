// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeTimeToLiveOutput {
    /// <p></p>
    #[doc(hidden)]
    pub time_to_live_description: std::option::Option<crate::types::TimeToLiveDescription>,
    _request_id: Option<String>,
}
impl DescribeTimeToLiveOutput {
    /// <p></p>
    pub fn time_to_live_description(
        &self,
    ) -> std::option::Option<&crate::types::TimeToLiveDescription> {
        self.time_to_live_description.as_ref()
    }
}
impl aws_http::request_id::RequestId for DescribeTimeToLiveOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTimeToLiveOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTimeToLiveOutput`](crate::operation::describe_time_to_live::DescribeTimeToLiveOutput).
    pub fn builder(
    ) -> crate::operation::describe_time_to_live::builders::DescribeTimeToLiveOutputBuilder {
        crate::operation::describe_time_to_live::builders::DescribeTimeToLiveOutputBuilder::default(
        )
    }
}

/// A builder for [`DescribeTimeToLiveOutput`](crate::operation::describe_time_to_live::DescribeTimeToLiveOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DescribeTimeToLiveOutputBuilder {
    pub(crate) time_to_live_description: std::option::Option<crate::types::TimeToLiveDescription>,
    _request_id: Option<String>,
}
impl DescribeTimeToLiveOutputBuilder {
    /// <p></p>
    pub fn time_to_live_description(mut self, input: crate::types::TimeToLiveDescription) -> Self {
        self.time_to_live_description = Some(input);
        self
    }
    /// <p></p>
    pub fn set_time_to_live_description(
        mut self,
        input: std::option::Option<crate::types::TimeToLiveDescription>,
    ) -> Self {
        self.time_to_live_description = input;
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
    /// Consumes the builder and constructs a [`DescribeTimeToLiveOutput`](crate::operation::describe_time_to_live::DescribeTimeToLiveOutput).
    pub fn build(self) -> crate::operation::describe_time_to_live::DescribeTimeToLiveOutput {
        crate::operation::describe_time_to_live::DescribeTimeToLiveOutput {
            time_to_live_description: self.time_to_live_description,
            _request_id: self._request_id,
        }
    }
}
