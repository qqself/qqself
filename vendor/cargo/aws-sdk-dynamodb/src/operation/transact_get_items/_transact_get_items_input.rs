// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TransactGetItemsInput {
    /// <p>An ordered array of up to 100 <code>TransactGetItem</code> objects, each of which contains a <code>Get</code> structure.</p>
    #[doc(hidden)]
    pub transact_items: std::option::Option<std::vec::Vec<crate::types::TransactGetItem>>,
    /// <p>A value of <code>TOTAL</code> causes consumed capacity information to be returned, and a value of <code>NONE</code> prevents that information from being returned. No other value is valid.</p>
    #[doc(hidden)]
    pub return_consumed_capacity: std::option::Option<crate::types::ReturnConsumedCapacity>,
}
impl TransactGetItemsInput {
    /// <p>An ordered array of up to 100 <code>TransactGetItem</code> objects, each of which contains a <code>Get</code> structure.</p>
    pub fn transact_items(&self) -> std::option::Option<&[crate::types::TransactGetItem]> {
        self.transact_items.as_deref()
    }
    /// <p>A value of <code>TOTAL</code> causes consumed capacity information to be returned, and a value of <code>NONE</code> prevents that information from being returned. No other value is valid.</p>
    pub fn return_consumed_capacity(
        &self,
    ) -> std::option::Option<&crate::types::ReturnConsumedCapacity> {
        self.return_consumed_capacity.as_ref()
    }
}
impl TransactGetItemsInput {
    /// Creates a new builder-style object to manufacture [`TransactGetItemsInput`](crate::operation::transact_get_items::TransactGetItemsInput).
    pub fn builder() -> crate::operation::transact_get_items::builders::TransactGetItemsInputBuilder
    {
        crate::operation::transact_get_items::builders::TransactGetItemsInputBuilder::default()
    }
}

/// A builder for [`TransactGetItemsInput`](crate::operation::transact_get_items::TransactGetItemsInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct TransactGetItemsInputBuilder {
    pub(crate) transact_items: std::option::Option<std::vec::Vec<crate::types::TransactGetItem>>,
    pub(crate) return_consumed_capacity: std::option::Option<crate::types::ReturnConsumedCapacity>,
}
impl TransactGetItemsInputBuilder {
    /// Appends an item to `transact_items`.
    ///
    /// To override the contents of this collection use [`set_transact_items`](Self::set_transact_items).
    ///
    /// <p>An ordered array of up to 100 <code>TransactGetItem</code> objects, each of which contains a <code>Get</code> structure.</p>
    pub fn transact_items(mut self, input: crate::types::TransactGetItem) -> Self {
        let mut v = self.transact_items.unwrap_or_default();
        v.push(input);
        self.transact_items = Some(v);
        self
    }
    /// <p>An ordered array of up to 100 <code>TransactGetItem</code> objects, each of which contains a <code>Get</code> structure.</p>
    pub fn set_transact_items(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TransactGetItem>>,
    ) -> Self {
        self.transact_items = input;
        self
    }
    /// <p>A value of <code>TOTAL</code> causes consumed capacity information to be returned, and a value of <code>NONE</code> prevents that information from being returned. No other value is valid.</p>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.return_consumed_capacity = Some(input);
        self
    }
    /// <p>A value of <code>TOTAL</code> causes consumed capacity information to be returned, and a value of <code>NONE</code> prevents that information from being returned. No other value is valid.</p>
    pub fn set_return_consumed_capacity(
        mut self,
        input: std::option::Option<crate::types::ReturnConsumedCapacity>,
    ) -> Self {
        self.return_consumed_capacity = input;
        self
    }
    /// Consumes the builder and constructs a [`TransactGetItemsInput`](crate::operation::transact_get_items::TransactGetItemsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::transact_get_items::TransactGetItemsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::transact_get_items::TransactGetItemsInput {
                transact_items: self.transact_items,
                return_consumed_capacity: self.return_consumed_capacity,
            },
        )
    }
}
