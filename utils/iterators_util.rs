
use std::marker::PhantomData;

pub struct TupleIteratable<'a, T> where T: 'a + IntoIterator {
    _obj: T::IntoIter,
    phantom: PhantomData<&'a T>, 
}

impl<'a, T> TupleIteratable<'a, T> where T: 'a + IntoIterator {
    pub fn new(obj: T) -> TupleIteratable<'a, T>
    {
        TupleIteratable{ _obj: obj.into_iter(), phantom: PhantomData{} }
    }
}

impl<'a, T> Iterator for TupleIteratable<'a, T> where T: 'a + IntoIterator {
    type Item = (T::Item,T::Item,T::Item,T::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let mut res: (Option<T::Item>, Option<T::Item>, Option<T::Item>, Option<T::Item>) = (None, None, None, None);
        let mut ok = true;
        match self._obj.next() {
            Some(x) => {res.0 = Some(x)},
            None => {ok = false}
        };
        if ok == true {
            match self._obj.next() {
                Some(x) => {res.1 = Some(x)},
                None => {ok = false}
            };
        }
        if ok == true {
            match self._obj.next() {
                Some(x) => {res.2 = Some(x)},
                None => {ok = false}
            };
        }
        if ok == true {
            match self._obj.next() {
                Some(x) => {res.3 = Some(x)},
                None => {ok = false}
            };
        }
        return if ok {Some((res.0.unwrap(), res.1.unwrap(), res.2.unwrap(), res.3.unwrap()))} else {None}
    }
}