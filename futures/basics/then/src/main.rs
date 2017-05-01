extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = ok::<i32, i32>(1);

    // NOTES: 
    // 1. then executes the closures regardless future's results. closure arg take full future unlike other
    //    combinators which take completed values (or skip altogether)
    // 2. doesn't care about item, error type matching b/w f1 & closure
    //    ( but 'Then's item, error type will be closure returned intofuture's item, error type)
    // 3. closure should be an intofuture
    let f = f1.then(|v| {
        println!("{:?}", v);
        Ok::<bool, i64>(true)
    });

    assert_eq!(Ok(true), reactor.run(f)); // Map poll will wrap 'true' to Ok(true)

    let f1 = err::<i32, i32>(-1);
    let f = f1.then(|v| {
        println!("{:?}", v);
        Err::<bool, i64>(-2)
    });

    assert_eq!(Err(-2), reactor.run(f));
}
