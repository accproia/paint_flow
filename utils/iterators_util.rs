

pub struct TupleIteratable<T> where T: IntoIterator {
    _obj: T::IntoIter,
}

impl<T> TupleIteratable<T> where T: IntoIterator {
    pub fn new(obj: T) -> TupleIteratable<T>
    {
        TupleIteratable{ _obj: obj.into_iter() }
    }
}

impl<T> Iterator for TupleIteratable<T> where T: IntoIterator {
    type Item = (T::Item,T::Item,T::Item,T::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let res = (
            self._obj.next(), self._obj.next(), self._obj.next(), self._obj.next()
        );
        return if res.3.is_some() {Some((res.0.unwrap(), res.1.unwrap(), res.2.unwrap(), res.3.unwrap()))} else {None}
    }
}


#[macro_export]
macro_rules! tuple_iteratable {
    ($x:expr) => (TupleIteratable::new($x));
}