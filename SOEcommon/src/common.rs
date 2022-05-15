pub enum SOErr{
    Err,
    Ok,
    NotSupported,
    OSNotSupported,
}

#[macro_export]
macro_rules! s {
    ($e:expr) => {
        $e.to_string()
    };
}
#[macro_export]
macro_rules! o {
    ($e:expr) => {
        $e.to_owned()
    };
}


