
macro_rules! print_item {
    ($item_type:ty, $number:tt) => ($item_type);
}

macro_rules! print_items {
    ($item_type:ty, $($number:tt),+) => (
        ($(print_item!($item_type, $number)),+,)
    );
}

macro_rules! print_one_res {
    ($ex:expr, $number:tt) => ($ex);
}

macro_rules! print_res {
    ($ex:expr, $($number:tt),+) => (
        ($(print_one_res!($ex, $number)),+,)
    );
}

macro_rules! print_one_unwrap_res {
    ($var_name:ident, $func:ident, $number:tt) => ($var_name.$number.$func());
}

macro_rules! print_unwrap_res {
    ($var_name:ident, $func:ident, $($number:tt),+) => (
        ($(print_one_unwrap_res!($var_name, $func, $number)),+,)
    );
}

macro_rules! check_last_element {
    ($var_name:ident, $func:ident, $number:tt) => (
        $var_name.$number.$func()
    );
    ($var_name:ident, $func:ident, $number:tt, $($numbers:tt),+) => (
        check_last_element!($var_name, $func, $($numbers),+)
    );
}

macro_rules! declare_tuple_iteratable {
    ($name:ident, $($numbers:tt),+) => {
        pub struct $name<T> where T: IntoIterator {
            _obj: T::IntoIter,
        }
        
        impl<T> $name<T> where T: IntoIterator {
            pub fn _new(obj: T) -> $name<T> {
                $name{ _obj: obj.into_iter() }
            }
        }
        
        impl<T> Iterator for $name<T> where T: IntoIterator {
            type Item = print_items!(T::Item, $($numbers),+);
            fn next(&mut self) -> Option<Self::Item> {
                let res = print_res!(self._obj.next(), $($numbers),+);
                return if check_last_element!(res, is_some, $($numbers),+) {
                    Some(print_unwrap_res!(res, unwrap, $($numbers),+))
                } else {None}
            }
        }
    };
}

declare_tuple_iteratable!(TupleIteratable1, 0);
declare_tuple_iteratable!(TupleIteratable2, 0, 1);
declare_tuple_iteratable!(TupleIteratable3, 0, 1, 2);
declare_tuple_iteratable!(TupleIteratable4, 0, 1, 2, 3);
declare_tuple_iteratable!(TupleIteratable5, 0, 1, 2, 3, 4);
declare_tuple_iteratable!(TupleIteratable6, 0, 1, 2, 3, 4, 5);
declare_tuple_iteratable!(TupleIteratable7, 0, 1, 2, 3, 4, 5, 6);
declare_tuple_iteratable!(TupleIteratable8, 0, 1, 2, 3, 4, 5, 6, 7);
declare_tuple_iteratable!(TupleIteratable9, 0, 1, 2, 3, 4, 5, 6, 7, 8);
declare_tuple_iteratable!(TupleIteratable10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
declare_tuple_iteratable!(TupleIteratable11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);


#[macro_export]
macro_rules! iterate_by_tuple {
    (1 of $x:expr) => (TupleIteratable1::_new($x));
    (2 of $x:expr) => (TupleIteratable2::_new($x));
    (3 of $x:expr) => (TupleIteratable3::_new($x));
    (4 of $x:expr) => (TupleIteratable4::_new($x));
    (5 of $x:expr) => (TupleIteratable5::_new($x));
    (6 of $x:expr) => (TupleIteratable6::_new($x));
    (7 of $x:expr) => (TupleIteratable7::_new($x));
    (8 of $x:expr) => (TupleIteratable8::_new($x));
    (9 of $x:expr) => (TupleIteratable9::_new($x));
    (10 of $x:expr) => (TupleIteratable10::_new($x));
}