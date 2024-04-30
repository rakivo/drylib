#[macro_export]
macro_rules! lock { // locks given variable whether it's mutex or not
    ($var: expr) => { $var.lock().expect("Failed to lock") };
    ($var: expr, $expect: expr) => { $var.lock().expect($expect) };
    ($var: ident:?) => { $var.lock().expect(&format!("Failed to lock {var}", var = stringify!($var))) };
}

#[macro_export]
macro_rules! parse { // parses one type into another
    ($var: expr, $type: ty) => { $var.parse::<$type>().expect("Failed to parse") };
    ($var: expr, $type: ty, $expect: expr) => { $var.parse::<$type>().expect($expect) };
    ($var: expr, $type: ty:?) => { $var.parse::<$type>().expect(&format!("Failed to parse {t1} into {t2}", t1 = $var, t2 = stringify!($type))) };
}

// am -> Arc Mutex
#[macro_export]
macro_rules! am { // creates Arc<Mutex<`your variable`>>
    ($var: expr) => { std::sync::Arc::new(std::sync::Mutex::new($var)) };
}

#[macro_export]
macro_rules! sleep { // puts thread on sleep
    (i|$t: expr) => { std::thread::sleep(std::time::Duration::from_secs($t as u64)) };
    ($t: expr) => { std::thread::sleep($t) };
}

#[macro_export]
macro_rules! read { // reads input from stdin. 
    // Use std::io::{Write, BufRead, BufReader} for it to work.
    ($msg: expr, $rbuf: expr => $buf: expr) => { // rbuf basically is std::io::BufReader
        print!("{m}", m = $msg); std::io::stdout().flush().unwrap();
        $rbuf.read_line($buf).ok();
    };
    ($rbuf: expr => $buf: expr) => {
        $rbuf.read_line($buf).ok();
    };
    ($buf: expr) => { stdin().read_line($buf).ok() };
}

#[macro_export]
macro_rules! pubstruct { // creates pub struct with optional generic types, optional lifetimes, with all of the fields are public as well.
    ($(#[$meta:meta])*
    $name: ident($(#[$fmeta:meta])* $($t: ty), *)) => {
        $(#[$meta])*
        pub struct $name($(#[$fmeta])* $(pub $t), *);
    };

    ($(#[$meta:meta])*
    $name: ident<$($T: ident), +>($(#[$fmeta:meta])* $($t: ty), *)) => {
        $(#[$meta])*
        pub struct $name<$($T,) *>($(#[$fmeta])* $(pub $t), *);
    };

    ($(#[$meta:meta])*
    $name: ident<$($L: lifetime), +>($(#[$fmeta:meta])* $($t: ty), *)) => {
        $(#[$meta])*
        pub struct $name<$($L,) *>($(#[$fmeta])* $(pub $t), *);
    };

    ($(#[$meta:meta])*
    $name: ident<$($L: lifetime), +, $($T: ident), +>($(#[$fmeta:meta])* $($t: ty), *)) => {
        $(#[$meta])*
        pub struct $name<$($L,) * $($T,) *>($(#[$fmeta])* $(pub $t), *);
    };

    ($(#[$meta:meta])*
    $name: ident {
        $($(#[$fmeta:meta])*
        $field: ident: $t: ty),* $(,)?
    }) => {
        $(#[$meta])*
        pub struct $name {
            $($(#[$fmeta])*
            pub $field: $t),*
        }
    };

    ($(#[$meta:meta])*
    $name: ident<$($T: ident),*> {
        $($(#[$fmeta:meta])*
        $field:ident: $t:ty),* $(,)?
    }) => {
        $(#[$meta])*
        pub struct $name<$($T),*> {
            $($(#[$fmeta])* pub $field: $t),*
        }
    };

    ($(#[$meta:meta])*
    $name: ident<$($L: lifetime),*> {
        $($(#[$fmeta:meta])*
        $field: ident: $t: ty),* $(,)?
    }) => {
        $(#[$meta])*
        pub struct $name<$($L),*> {
            $($(#[$fmeta])* pub $field: $t),*
        }
    };

    ($(#[$meta: meta])*
    $name: ident<$($L: lifetime),*, $($T: ident),*> {
        $($(#[$fmeta: meta])*
        $field: ident: $t: ty),* $(,)?
    }) => {
        $(#[$meta])*
        pub struct $name<$($L),*, $($T),*> {
            $($(#[$fmeta])* pub $field: $t),*
        }
    };
}

#[macro_export]
macro_rules! impl_getters {
    ($(#[$meta: meta])*
    $vis: vis,
    $name: ident {
        $($field: ident: $t: ty),*
    }) => {
        $(#[$meta])*
        impl $name {
            $($vis fn $field(&self) -> &$t {
                &self.$field
            })*
        }
    };
}

#[macro_export]
macro_rules! colored {
    (pr | $($args: tt), *) => { // pr -> print red 
        println!("\x1b[31m{}\x1b[0m", format_args!($($args)*))
    };
    (fr | $($args: tt), *) => { // fr -> format red
        format!("\x1b[31m{}\x1b[0m", format_args!($($args)*))
    };
    (pg | $($args: tt), *) => { // pg -> print gray 
        println!("\x1b[90m{}\x1b[0m", format_args!($($args)*))
    };
    (fg | $($args: tt), *) => { // fg -> format gray
        format!("\x1b[90m{}\x1b[0m", format_args!($($args)*))
    };
}
