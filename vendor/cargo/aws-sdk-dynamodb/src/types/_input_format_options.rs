// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The format options for the data that was imported into the target table. There is one value, CsvOption.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InputFormatOptions {
    /// <p> The options for imported source files in CSV format. The values are Delimiter and HeaderList. </p>
    #[doc(hidden)]
    pub csv: std::option::Option<crate::types::CsvOptions>,
}
impl InputFormatOptions {
    /// <p> The options for imported source files in CSV format. The values are Delimiter and HeaderList. </p>
    pub fn csv(&self) -> std::option::Option<&crate::types::CsvOptions> {
        self.csv.as_ref()
    }
}
impl InputFormatOptions {
    /// Creates a new builder-style object to manufacture [`InputFormatOptions`](crate::types::InputFormatOptions).
    pub fn builder() -> crate::types::builders::InputFormatOptionsBuilder {
        crate::types::builders::InputFormatOptionsBuilder::default()
    }
}

/// A builder for [`InputFormatOptions`](crate::types::InputFormatOptions).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct InputFormatOptionsBuilder {
    pub(crate) csv: std::option::Option<crate::types::CsvOptions>,
}
impl InputFormatOptionsBuilder {
    /// <p> The options for imported source files in CSV format. The values are Delimiter and HeaderList. </p>
    pub fn csv(mut self, input: crate::types::CsvOptions) -> Self {
        self.csv = Some(input);
        self
    }
    /// <p> The options for imported source files in CSV format. The values are Delimiter and HeaderList. </p>
    pub fn set_csv(mut self, input: std::option::Option<crate::types::CsvOptions>) -> Self {
        self.csv = input;
        self
    }
    /// Consumes the builder and constructs a [`InputFormatOptions`](crate::types::InputFormatOptions).
    pub fn build(self) -> crate::types::InputFormatOptions {
        crate::types::InputFormatOptions { csv: self.csv }
    }
}
