#[cfg(feature = "semihosting")]

pub use cortex_m_semihosting::hprintln;

#[macro_export]
macro_rules! sprintln {
    () => {
        #[cfg(feature = "semihosting")]
        hprintln!().unwrap();
    };
    ($s:expr) => {
        #[cfg(feature = "semihosting")]
        hprintln!($s).unwrap();
    };
    ($s:expr, $($tt:tt)*) => {
        #[cfg(feature = "semihosting")]
        hprintln!($s, $($tt)*).unwrap();
  };
}

#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        #[cfg(feature = "semihosting")]
        hprint!($s).unwrap();
    };
    ($($tt:tt)*) => {
        #[cfg(feature = "semihosting")]
        hprint!($($tt)*).unwrap();
    };
}

#[macro_export]
macro_rules! seprintln {
    () => {
        #[cfg(feature = "semihosting")]
        heprintln!().unwrap();
    };
    ($s:expr) => {
        #[cfg(feature = "semihosting")]
        heprintln!($s).unwrap();
    };
    ($s:expr, $($tt:tt)*) => {
        #[cfg(feature = "semihosting")]
        heprintln!($s, $($tt)*).unwrap();
  };
}

#[macro_export]
macro_rules! seprint {
    ($s:expr) => {
        #[cfg(feature = "semihosting")]
        heprint!($s).unwrap();
    };
    ($($tt:tt)*) => {
        #[cfg(feature = "semihosting")]
        heprint!($($tt)*).unwrap();
    };
}
