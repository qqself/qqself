// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_continuous_backups_input(
    input: &crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_continuous_backups_input::ser_describe_continuous_backups_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_continuous_backups_http_error(
    _response_status: u16,
    _response_headers: &http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
    crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output).map_err(crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidEndpointException" => crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::InvalidEndpointException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidEndpointExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output).map_err(crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TableNotFoundException" => crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::TableNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TableNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_table_not_found_exception::de_table_not_found_exception_json_err(_response_body, output).map_err(crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_continuous_backups_http_response(
    _response_status: u16,
    _response_headers: &http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
    crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_continuous_backups::de_describe_continuous_backups(_response_body, output).map_err(crate::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_continuous_backups(
    value: &[u8],
    mut builder: crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsOutputBuilder,
) -> Result<
    crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ContinuousBackupsDescription" => {
                        builder = builder.set_continuous_backups_description(
                            crate::protocol_serde::shape_continuous_backups_description::de_continuous_backups_description(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
