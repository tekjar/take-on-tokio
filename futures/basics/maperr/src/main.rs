extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = err::<i32, i32>(-1);
    
    // MapErr Combinator:
    // ------------------
    //
    // fn map_err<F, E>(self, f: F) -> MapErr<Self, F> 
    // where                             * F get's executed when Self(Future) resolves to error .
    //   F: FnOnce(Self::Error) -> E,    * No restrictions on return type. map_err() will envolope the closure return with 'Err'.
    //                                     So unlike 'andthen' and 'orelse', map_err will always resolves to 'Err' when Self is Err
    //   Self: Sized, 

    // NOTES: 
    // 1. maperr executes the closures only when future results err or else passes the future along unaltered
    // 2. doesn't care about item, error type matching b/w f1 & closure
    // 3. closure return value doesn't have to be intofuture
    //    ( but MapErr will take care of converting closure return type to Result (which is an intofuture) 
    //      Err::<f1's item type, _>(closure return) /) (f1's item type doesn't matter though as this always results in error)
    let f = f1.map_err(|v| {
        println!("{:?}", v);
        true // (2), (3)
    });

    assert_eq!(Err(true), reactor.run(f)); // Map poll will wrap 'true' to Ok(true)

    let f1 = err::<i32, i32>(-1);
    let f = f1.map_err(|v| {
        println!("{:?}", v);
        Ok::<_, ()>(true)
    });

    assert_eq!(Err(Ok(true)), reactor.run(f));

    let f1 = ok::<i32, i32>(1);
    let f = f1.map_err(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok::<_, ()>(true)
    });

    assert_eq!(Ok(1), reactor.run(f));
}