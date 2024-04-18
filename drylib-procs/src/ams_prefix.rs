#[cfg(feature = "ams-prefix-m")]
pub(crate) const AMS_PREFIX: char = 'm';
#[cfg(not(feature = "ams-prefix-m"))]
#[cfg(feature = "ams-prefix-mu")]
pub(crate) const AMS_PREFIX: &str = "mu";
#[cfg(not(any(feature = "ams-prefix-m", feature = "ams-prefix-mu")))]
#[cfg(feature = "ams-prefix-mut")]
pub(crate) const AMS_PREFIX: &str = "mut";
#[cfg(not(any(feature = "ams-prefix-m", feature = "ams-prefix-mu", feature = "ams-prefix-mut")))]
#[cfg(feature = "ams-prefix-mute")]
pub(crate) const AMS_PREFIX: &str = "mute";
#[cfg(not(any(feature = "ams-prefix-m", feature = "ams-prefix-mu", feature = "ams-prefix-mut", feature = "ams-prefix-mute")))]
#[cfg(feature = "ams-prefix-mutex")]
pub(crate) const AMS_PREFIX: &str = "mutex";
#[cfg(not(any(feature = "ams-prefix-m", feature = "ams-prefix-mu", feature = "ams-prefix-mut", feature = "ams-prefix-mute", feature = "ams-prefix-mutex")))]
pub(crate) const AMS_PREFIX: &str = "m";
