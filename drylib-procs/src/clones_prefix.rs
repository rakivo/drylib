#[cfg(feature = "clones-prefix-c")]
pub(crate) const CLONES_PREFIX: char = 'c';
#[cfg(not(feature = "clones-prefix-c"))]
#[cfg(feature = "clones-prefix-cl")]
pub(crate) const CLONES_PREFIX: &str = "cl";
#[cfg(not(any(feature = "clones-prefix-c", feature = "clones-prefix-cl")))]
#[cfg(feature = "clones-prefix-clo")]
pub(crate) const CLONES_PREFIX: &str = "clo";
#[cfg(not(any(feature = "clones-prefix-c", feature = "clones-prefix-cl", feature = "clones-prefix-clo")))]
#[cfg(feature = "clones-prefix-clon")]
pub(crate) const CLONES_PREFIX: &str = "clon";
#[cfg(not(any(feature = "clones-prefix-c", feature = "clones-prefix-cl", feature = "clones-prefix-clo", feature = "clones-prefix-clon")))]
#[cfg(feature = "clones-prefix-clone")]
pub(crate) const CLONES_PREFIX: &str = "clone";
#[cfg(not(any(feature = "clones-prefix-c", feature = "clones-prefix-cl", feature = "clones-prefix-clo", feature = "clones-prefix-clon", feature = "clones-prefix-clone")))]
pub(crate) const CLONES_PREFIX: &str = "c";
