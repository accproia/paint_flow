


// pub struct TupleIteratable<'a, T> where T: 'a, T: Iterator {
//     _obj: &'a mut T,
// }

// impl<'a, T> TupleIteratable<'a, T> where T: 'a, T: Iterator {
//     pub fn new(obj: &'a mut T) -> TupleIteratable<'a, T>
//     {
//         TupleIteratable{ _obj: obj }
//     }
// }

// impl<'a, T> Iterator for TupleIteratable<'a, T> where T: 'a, T: Iterator {
//     type Item = (&'a mut T::Item,);
//     fn next(&mut self) -> Option<Self::Item> {
//         match self._obj.next() {
//             Some(x) => Some((&mut x,)),
//             None => None
//         }
//     }
// }