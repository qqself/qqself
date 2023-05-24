// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to perform an <code>UpdateItem</code> operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Update {
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
    #[doc(hidden)]
    pub key: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new value(s) for them.</p>
    #[doc(hidden)]
    pub update_expression: std::option::Option<std::string::String>,
    /// <p>Name of the table for the <code>UpdateItem</code> request.</p>
    #[doc(hidden)]
    pub table_name: std::option::Option<std::string::String>,
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p>
    #[doc(hidden)]
    pub condition_expression: std::option::Option<std::string::String>,
    /// <p>One or more substitution tokens for attribute names in an expression.</p>
    #[doc(hidden)]
    pub expression_attribute_names:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>One or more values that can be substituted in an expression.</p>
    #[doc(hidden)]
    pub expression_attribute_values: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>Update</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE, ALL_OLD, UPDATED_OLD, ALL_NEW, UPDATED_NEW.</p>
    #[doc(hidden)]
    pub return_values_on_condition_check_failure:
        std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
}
impl Update {
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn key(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    > {
        self.key.as_ref()
    }
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new value(s) for them.</p>
    pub fn update_expression(&self) -> std::option::Option<&str> {
        self.update_expression.as_deref()
    }
    /// <p>Name of the table for the <code>UpdateItem</code> request.</p>
    pub fn table_name(&self) -> std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p>
    pub fn condition_expression(&self) -> std::option::Option<&str> {
        self.condition_expression.as_deref()
    }
    /// <p>One or more substitution tokens for attribute names in an expression.</p>
    pub fn expression_attribute_names(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.expression_attribute_names.as_ref()
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    pub fn expression_attribute_values(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    > {
        self.expression_attribute_values.as_ref()
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>Update</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE, ALL_OLD, UPDATED_OLD, ALL_NEW, UPDATED_NEW.</p>
    pub fn return_values_on_condition_check_failure(
        &self,
    ) -> std::option::Option<&crate::types::ReturnValuesOnConditionCheckFailure> {
        self.return_values_on_condition_check_failure.as_ref()
    }
}
impl Update {
    /// Creates a new builder-style object to manufacture [`Update`](crate::types::Update).
    pub fn builder() -> crate::types::builders::UpdateBuilder {
        crate::types::builders::UpdateBuilder::default()
    }
}

/// A builder for [`Update`](crate::types::Update).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct UpdateBuilder {
    pub(crate) key: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
    pub(crate) update_expression: std::option::Option<std::string::String>,
    pub(crate) table_name: std::option::Option<std::string::String>,
    pub(crate) condition_expression: std::option::Option<std::string::String>,
    pub(crate) expression_attribute_names:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) expression_attribute_values: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
    pub(crate) return_values_on_condition_check_failure:
        std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
}
impl UpdateBuilder {
    /// Adds a key-value pair to `key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn key(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        let mut hash_map = self.key.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.key = Some(hash_map);
        self
    }
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn set_key(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.key = input;
        self
    }
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new value(s) for them.</p>
    pub fn update_expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.update_expression = Some(input.into());
        self
    }
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new value(s) for them.</p>
    pub fn set_update_expression(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.update_expression = input;
        self
    }
    /// <p>Name of the table for the <code>UpdateItem</code> request.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.table_name = Some(input.into());
        self
    }
    /// <p>Name of the table for the <code>UpdateItem</code> request.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p>
    pub fn condition_expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.condition_expression = Some(input.into());
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p>
    pub fn set_condition_expression(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.condition_expression = input;
        self
    }
    /// Adds a key-value pair to `expression_attribute_names`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in an expression.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.expression_attribute_names.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.expression_attribute_names = Some(hash_map);
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.expression_attribute_names = input;
        self
    }
    /// Adds a key-value pair to `expression_attribute_values`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
    ///
    /// <p>One or more values that can be substituted in an expression.</p>
    pub fn expression_attribute_values(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        let mut hash_map = self.expression_attribute_values.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.expression_attribute_values = Some(hash_map);
        self
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    pub fn set_expression_attribute_values(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.expression_attribute_values = input;
        self
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>Update</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE, ALL_OLD, UPDATED_OLD, ALL_NEW, UPDATED_NEW.</p>
    pub fn return_values_on_condition_check_failure(
        mut self,
        input: crate::types::ReturnValuesOnConditionCheckFailure,
    ) -> Self {
        self.return_values_on_condition_check_failure = Some(input);
        self
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>Update</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE, ALL_OLD, UPDATED_OLD, ALL_NEW, UPDATED_NEW.</p>
    pub fn set_return_values_on_condition_check_failure(
        mut self,
        input: std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
    ) -> Self {
        self.return_values_on_condition_check_failure = input;
        self
    }
    /// Consumes the builder and constructs a [`Update`](crate::types::Update).
    pub fn build(self) -> crate::types::Update {
        crate::types::Update {
            key: self.key,
            update_expression: self.update_expression,
            table_name: self.table_name,
            condition_expression: self.condition_expression,
            expression_attribute_names: self.expression_attribute_names,
            expression_attribute_values: self.expression_attribute_values,
            return_values_on_condition_check_failure: self.return_values_on_condition_check_failure,
        }
    }
}