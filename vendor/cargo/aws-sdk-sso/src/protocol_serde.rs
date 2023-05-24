// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_get_role_credentials;

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &http::HeaderMap,
    response_body: &[u8],
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_list_account_roles;

pub(crate) mod shape_list_accounts;

pub(crate) mod shape_logout;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_invalid_request_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_unauthorized_exception;

pub(crate) mod shape_account_list_type;

pub(crate) mod shape_role_credentials;

pub(crate) mod shape_role_list_type;

pub(crate) mod shape_account_info;

pub(crate) mod shape_role_info;
