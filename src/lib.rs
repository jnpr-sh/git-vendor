//! In-source vendoring for Git repositories.
//!
//! Vendor dependencies are tracked via custom attributes in `.gitattributes`:
//!
//! ```text
//! path/to/dep/* vendored name=owner/repo url=https://example.com/owner/repo.git branch=main
//! ```
