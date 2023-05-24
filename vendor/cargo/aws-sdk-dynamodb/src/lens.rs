// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_contributor_insights_output_next_token(
    input: &crate::operation::list_contributor_insights::ListContributorInsightsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_exports_output_next_token(
    input: &crate::operation::list_exports::ListExportsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_imports_output_next_token(
    input: &crate::operation::list_imports::ListImportsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_tables_output_last_evaluated_table_name(
    input: &crate::operation::list_tables::ListTablesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.last_evaluated_table_name {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_query_output_last_evaluated_key(
    input: &crate::operation::query::QueryOutput,
) -> std::option::Option<
    &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
> {
    let input = match &input.last_evaluated_key {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_scan_output_last_evaluated_key(
    input: &crate::operation::scan::ScanOutput,
) -> std::option::Option<
    &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
> {
    let input = match &input.last_evaluated_key {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_tables_output_table_names(
    input: crate::operation::list_tables::ListTablesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.table_names {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_query_output_items(
    input: crate::operation::query::QueryOutput,
) -> std::option::Option<
    std::vec::Vec<std::collections::HashMap<std::string::String, crate::types::AttributeValue>>,
> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_scan_output_items(
    input: crate::operation::scan::ScanOutput,
) -> std::option::Option<
    std::vec::Vec<std::collections::HashMap<std::string::String, crate::types::AttributeValue>>,
> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
