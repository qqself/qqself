// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `BackupTypeFilter`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let backuptypefilter = unimplemented!();
/// match backuptypefilter {
///     BackupTypeFilter::All => { /* ... */ },
///     BackupTypeFilter::AwsBackup => { /* ... */ },
///     BackupTypeFilter::System => { /* ... */ },
///     BackupTypeFilter::User => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `backuptypefilter` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `BackupTypeFilter::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `BackupTypeFilter::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `BackupTypeFilter::NewFeature` is defined.
/// Specifically, when `backuptypefilter` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `BackupTypeFilter::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum BackupTypeFilter {
    #[allow(missing_docs)] // documentation missing in model
    All,
    #[allow(missing_docs)] // documentation missing in model
    AwsBackup,
    #[allow(missing_docs)] // documentation missing in model
    System,
    #[allow(missing_docs)] // documentation missing in model
    User,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for BackupTypeFilter {
    fn from(s: &str) -> Self {
        match s {
            "ALL" => BackupTypeFilter::All,
            "AWS_BACKUP" => BackupTypeFilter::AwsBackup,
            "SYSTEM" => BackupTypeFilter::System,
            "USER" => BackupTypeFilter::User,
            other => {
                BackupTypeFilter::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for BackupTypeFilter {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(BackupTypeFilter::from(s))
    }
}
impl BackupTypeFilter {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            BackupTypeFilter::All => "ALL",
            BackupTypeFilter::AwsBackup => "AWS_BACKUP",
            BackupTypeFilter::System => "SYSTEM",
            BackupTypeFilter::User => "USER",
            BackupTypeFilter::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["ALL", "AWS_BACKUP", "SYSTEM", "USER"]
    }
}
impl AsRef<str> for BackupTypeFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
