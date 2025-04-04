//! # `DBus` interface proxy for: `org.a11y.atspi.Cache`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Cache.xml`.
//!

use crate::common::{CacheItem, LegacyCacheItem};

#[zbus::proxy(interface = "org.a11y.atspi.Cache", default_path = "/org/a11y/atspi/cache")]
pub trait Cache {
	/// GetItems method
	fn get_items(&self) -> zbus::Result<Vec<CacheItem>>;

	/// GetItems method to support legacy servers (presumably Qt based applications and at-spi2-registryd)
	#[zbus(name = "GetItems")]
	fn get_legacy_items(&self) -> zbus::Result<Vec<LegacyCacheItem>>;
}
