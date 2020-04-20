
#[macro_export]
macro_rules! multiply {
    ($last:expr) => ($last);
    ($first:expr, $($others:expr),+) => ($first * multiply!($($others),+)); 
}

#[macro_export]
macro_rules! get_number_of_variadic_arguments {
    ($some_ident:ident) => (1);
    ($some_ident:ident, $($another:ident),+) => (1+get_number_of_variadic_arguments!($($another),+));
}

#[macro_export]
macro_rules! last_one {
    ($first:expr) => ($first);
    ($first:expr, $($another:expr),+) => (last_one!($($another),+));
}

#[macro_export]
macro_rules! coord_to_shift {
    ({$($sizes:expr),+}, $sizes_arr:expr, {$first:expr, $($others:expr),+}) => 
    ($first * multiply!($($sizes_arr[$sizes]),+) + coord_to_shift!({$($sizes),+,1+last_one!($($sizes),+)}, $sizes_arr, {$($others),+})); 
    ({$($sizes:expr),+}, $sizes_arr:expr, {$first:expr}) => 
    ($first * multiply!($($sizes_arr[$sizes]),+)); 
    ($sizes_arr:expr, $first:expr) => ($first); 
    ($sizes_arr:expr, $first:expr, $($others:expr),+) => ($first + coord_to_shift!({0}, $sizes_arr, {$($others),+}));
}