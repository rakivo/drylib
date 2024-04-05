#[macro_export]
macro_rules! lock { // locks given variable whether it's mutex or not
    ($var: expr) => { $var.lock().expect("Failed to lock") };
    ($var: expr, $expect: literal) => { $var.lock().expect($expect) };
    ($var: ident;e) => { $var.lock().expect(&format!("Failed to lock {var}", var = stringify!($var))) };
}

#[macro_export]
macro_rules! parse { // parses one type into another
    ($var: expr, $type: ty) => { $var.parse::<$type>().expect("Failed to parse") };
    ($var: expr, $type: ty, $expect: literal) => { $var.parse::<$type>().expect($expect) };
    ($var: expr, $type: ty;e) => { $var.parse::<$type>().expect(&format!("Failed to parse {t1} into {t2}", t1 = $var, t2 = stringify!($type))) };
}

// am -> Arc Mutex
#[macro_export]
macro_rules! am { // creates Arc<Mutex<`your variable`>>
    ($var: expr) => { Arc::new(Mutex::new($var)) };
}

#[macro_export]
macro_rules! sleep { // puts thread on sleep
    (i $t: expr) => { thread::sleep(Duration::from_secs($t as u64)) };
    (d $t: expr) => { thread::sleep($t) };
}

#[macro_export]
macro_rules! read { // reads input from stdin. 
    // Use std::io::{Write, BufRead, BufReader} for it to work.
    ($msg: expr, $buf: ident <- $rbuf: expr) => { // rbuf basically is std::io::BufReader
        print!("{m}", m = $msg); std::io::stdout().flush().unwrap();
        $rbuf.read_line(&mut $buf).ok();
        $buf = $buf.trim().to_owned()
    };
    ($buf: ident <- $rbuf: expr) => {
        $rbuf.read_line(&mut $buf).ok();
        $buf = $buf.trim().to_owned()
    };
    (1 $buf: expr) => { stdin().read_line(&mut $buf).ok() };
    (1b $buf: expr, $rbuf: ident) => { $rbuf.read_line(&mut $buf).ok() };
}

#[macro_export]
macro_rules! pub_tuple_struct {
(   $(#[$meta:meta])*
    $name: ident($($t: ty), *)
) => {
        $(#[$meta])*
        pub struct $name($(pub $t), *);
    };
}

#[macro_export]
macro_rules! pub_struct { // creates pub struct with all of the fields are public as well.
(   $(#[$meta:meta])*
    $name: ident {
       $($field: ident: $t: ty,) *
    }
) => {
        $(#[$meta])*
        pub struct $name {
            $(pub $field: $t), *
        }
    }
}

#[macro_export]
macro_rules! pubstructT { // creates pub struct with generic types, with all of the fields are public as well.
(   $(#[$meta:meta])*
    $name: ident<$($T: ident), +> {
        $($field: ident: $t:ty,) *
    }
) => {
        $(#[$meta])*
        pub struct $name<$($T),*> {
            $(pub $field: $t), *
        }
    }
}

#[macro_export]
macro_rules! pubstructLT { // creates pub struct with generic types and lifetimes, with all of the fields are public as well.
(   $(#[$meta:meta])*
    $name: ident<$($L: lifetime), +, $($T: ident), +> {
        $($field: ident: $t:ty,) *
    }
) => {
        $(#[$meta])*
        pub struct $name<$($L,)* $($T,)*> {
            $(pub $field: $t), *
        }
    }
}
