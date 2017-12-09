extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = ok::<i32, i32>(1);
    
    // Map Combinator:
    // ---------------
    //
    // fn map<F, U>(self, f: F) -> Map<Self, F> 
    // where                         * F get's executed when Self(Future) resolves to success .
    // F: FnOnce(Self::Item) -> U,   * No restrictions on F's return type. map() will envolope the closure return with 'Ok'. 
    //                                 So unlike 'andthen' and 'orelse', map will always resolves to 'Ok' when Self in succesful
    // Self: Sized,                  

    // NOTES: 
    // 1. Map executes the closure only when self results ok or else passes the result along unaltered
    // 2. doesn't care about item, error type matching b/w f1 & closure
    // 3. closure return value doesn't have to be intofuture
    //    ( but Map will take care of converting closure return type to Result (which is an intofuture) by
    //      with Ok::<_, f1's error type>(closure return) /) (f1's error type doesn't matter though as this always results in Ok)
    let f = f1.map(|v| {
        println!("{:?}", v);
        true // (2), (3)
    });

    assert_eq!(Ok(true), reactor.run(f)); // Map's poll will wrap 'true' to Ok(true)

    let f1 = ok::<i32, i32>(1);
    let f = f1.map(|v| {
        println!("{:?}", v);
        Ok::<_, ()>(true)
    });

    assert_eq!(Ok(Ok(true)), reactor.run(f));
    
    let f1 = ok::<i32, i32>(1);
    let f = f1.map(|v| {
        println!("{:?}", v);
        Err::<(), _>(-1)
    });

    assert_eq!(Ok(Err(-1)), reactor.run(f));

    let f1 = err::<i32, i32>(-1);
    let f = f1.map(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok::<_, ()>(true)
    });

    assert_eq!(Err(-1), reactor.run(f));
}