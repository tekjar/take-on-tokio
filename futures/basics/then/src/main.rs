extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = ok::<i32, i32>(1);

    // Then Combinator
    // ---------------
    //
    // fn then<F, B>(self, f: F) -> Then<Self, B, F> 
    // where                                                * F get's executed when Self(Future) resolves to error (or) success
    //   F: FnOnce(Result<Self::Item, Self::Error>) -> B,   * No restrictions on return type of the closure
    //   B: IntoFuture,                                     * but closure return should be IntoFuture
    //   Self: Sized,

    // NOTES: 
    // 1. Then executes the closure regardless of future's result. Unlike other
    //    combinators which gets executed based on future's success/failure, Then always executes the closure
    // 2. doesn't care about item, error type matching b/w f1 & closure
    //    ( but 'Then's item, error type will be closure returned intofuture's item, error type)
    // 3. closure return should be an intofuture
    let f = f1.then(|v| {
        println!("{:?}", v);
        Ok::<bool, i64>(true)
    });

    assert_eq!(Ok(true), reactor.run(f)); // Then poll will return Ok(true)

    let f1 = err::<i32, i32>(-1);
    let f = f1.then(|v| {
        println!("{:?}", v);
        Err::<bool, i64>(-2)
    });

    assert_eq!(Err(-2), reactor.run(f));
}
