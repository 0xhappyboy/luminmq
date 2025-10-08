#[macro_export]
macro_rules! exec_consume {
    ($fn_name:ident, $arg:expr) => {
        $fn_name($arg)
    };
}