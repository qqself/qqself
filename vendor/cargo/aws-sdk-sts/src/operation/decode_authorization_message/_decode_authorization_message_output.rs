// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A document that contains additional information about the authorization status of a request from an encoded message that is returned in response to an Amazon Web Services request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DecodeAuthorizationMessageOutput {
    /// <p>The API returns a response with the decoded message.</p>
    #[doc(hidden)]
    pub decoded_message: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DecodeAuthorizationMessageOutput {
    /// <p>The API returns a response with the decoded message.</p>
    pub fn decoded_message(&self) -> std::option::Option<&str> {
        self.decoded_message.as_deref()
    }
}
impl aws_http::request_id::RequestId for DecodeAuthorizationMessageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DecodeAuthorizationMessageOutput {
    /// Creates a new builder-style object to manufacture [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput).
    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder{
        crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default()
    }
}

/// A builder for [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DecodeAuthorizationMessageOutputBuilder {
    pub(crate) decoded_message: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DecodeAuthorizationMessageOutputBuilder {
    /// <p>The API returns a response with the decoded message.</p>
    pub fn decoded_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.decoded_message = Some(input.into());
        self
    }
    /// <p>The API returns a response with the decoded message.</p>
    pub fn set_decoded_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.decoded_message = input;
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
    /// Consumes the builder and constructs a [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput).
    pub fn build(
        self,
    ) -> crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput {
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput {
            decoded_message: self.decoded_message,
            _request_id: self._request_id,
        }
    }
}
