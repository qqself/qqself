// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::query::QueryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.table_name {
        object.key("TableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.index_name {
        object.key("IndexName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.select {
        object.key("Select").string(var_3.as_str());
    }
    if let Some(var_4) = &input.attributes_to_get {
        let mut array_5 = object.key("AttributesToGet").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.consistent_read {
        object.key("ConsistentRead").boolean(*var_8);
    }
    if let Some(var_9) = &input.key_conditions {
        #[allow(unused_mut)]
        let mut object_10 = object.key("KeyConditions").start_object();
        for (key_11, value_12) in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_13 = object_10.key(key_11.as_str()).start_object();
                crate::protocol_serde::shape_condition::ser_condition(&mut object_13, value_12)?;
                object_13.finish();
            }
        }
        object_10.finish();
    }
    if let Some(var_14) = &input.query_filter {
        #[allow(unused_mut)]
        let mut object_15 = object.key("QueryFilter").start_object();
        for (key_16, value_17) in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_18 = object_15.key(key_16.as_str()).start_object();
                crate::protocol_serde::shape_condition::ser_condition(&mut object_18, value_17)?;
                object_18.finish();
            }
        }
        object_15.finish();
    }
    if let Some(var_19) = &input.conditional_operator {
        object.key("ConditionalOperator").string(var_19.as_str());
    }
    if let Some(var_20) = &input.scan_index_forward {
        object.key("ScanIndexForward").boolean(*var_20);
    }
    if let Some(var_21) = &input.exclusive_start_key {
        #[allow(unused_mut)]
        let mut object_22 = object.key("ExclusiveStartKey").start_object();
        for (key_23, value_24) in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_25 = object_22.key(key_23.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(
                    &mut object_25,
                    value_24,
                )?;
                object_25.finish();
            }
        }
        object_22.finish();
    }
    if let Some(var_26) = &input.return_consumed_capacity {
        object.key("ReturnConsumedCapacity").string(var_26.as_str());
    }
    if let Some(var_27) = &input.projection_expression {
        object.key("ProjectionExpression").string(var_27.as_str());
    }
    if let Some(var_28) = &input.filter_expression {
        object.key("FilterExpression").string(var_28.as_str());
    }
    if let Some(var_29) = &input.key_condition_expression {
        object.key("KeyConditionExpression").string(var_29.as_str());
    }
    if let Some(var_30) = &input.expression_attribute_names {
        #[allow(unused_mut)]
        let mut object_31 = object.key("ExpressionAttributeNames").start_object();
        for (key_32, value_33) in var_30 {
            {
                object_31.key(key_32.as_str()).string(value_33.as_str());
            }
        }
        object_31.finish();
    }
    if let Some(var_34) = &input.expression_attribute_values {
        #[allow(unused_mut)]
        let mut object_35 = object.key("ExpressionAttributeValues").start_object();
        for (key_36, value_37) in var_34 {
            {
                #[allow(unused_mut)]
                let mut object_38 = object_35.key(key_36.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(
                    &mut object_38,
                    value_37,
                )?;
                object_38.finish();
            }
        }
        object_35.finish();
    }
    Ok(())
}
