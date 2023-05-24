// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_session_token_input_input(
    input: &crate::operation::get_session_token::GetSessionTokenInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetSessionToken", "2011-06-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DurationSeconds");
    if let Some(var_2) = &input.duration_seconds {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SerialNumber");
    if let Some(var_4) = &input.serial_number {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TokenCode");
    if let Some(var_6) = &input.token_code {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
