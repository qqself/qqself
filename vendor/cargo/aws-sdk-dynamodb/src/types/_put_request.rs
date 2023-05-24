// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to perform a <code>PutItem</code> operation on an item.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutRequest {
    /// <p>A map of attribute name to attribute values, representing the primary key of an item to be processed by <code>PutItem</code>. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema. If any attributes are present in the item that are part of an index key schema for the table, their types must match the index key schema.</p>
    #[doc(hidden)]
    pub item: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
}
impl PutRequest {
    /// <p>A map of attribute name to attribute values, representing the primary key of an item to be processed by <code>PutItem</code>. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema. If any attributes are present in the item that are part of an index key schema for the table, their types must match the index key schema.</p>
    pub fn item(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    > {
        self.item.as_ref()
    }
}
impl PutRequest {
    /// Creates a new builder-style object to manufacture [`PutRequest`](crate::types::PutRequest).
    pub fn builder() -> crate::types::builders::PutRequestBuilder {
        crate::types::builders::PutRequestBuilder::default()
    }
}

/// A builder for [`PutRequest`](crate::types::PutRequest).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct PutRequestBuilder {
    pub(crate) item: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
}
impl PutRequestBuilder {
    /// Adds a key-value pair to `item`.
    ///
    /// To override the contents of this collection use [`set_item`](Self::set_item).
    ///
    /// <p>A map of attribute name to attribute values, representing the primary key of an item to be processed by <code>PutItem</code>. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema. If any attributes are present in the item that are part of an index key schema for the table, their types must match the index key schema.</p>
    pub fn item(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        let mut hash_map = self.item.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.item = Some(hash_map);
        self
    }
    /// <p>A map of attribute name to attribute values, representing the primary key of an item to be processed by <code>PutItem</code>. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema. If any attributes are present in the item that are part of an index key schema for the table, their types must match the index key schema.</p>
    pub fn set_item(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.item = input;
        self
    }
    /// Consumes the builder and constructs a [`PutRequest`](crate::types::PutRequest).
    pub fn build(self) -> crate::types::PutRequest {
        crate::types::PutRequest { item: self.item }
    }
}
