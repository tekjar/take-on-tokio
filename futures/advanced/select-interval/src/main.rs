extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use std::thread;
use std::time::Duration;

use futures::stream::Stream;
use futures::{Future, Sink};
use futures::sync::mpsc;

use tokio_core::reactor::Core;
use tokio_timer::Timer;


fn main() {
    let mut reactor = Core::new().unwrap();
    // let handle = main_loop.handle();

    let (mut tx, rx) = mpsc::channel::<i32>(16);

    let timer = Timer::default();
    let interval = timer.interval(Duration::new(1, 0));

    // rx future has item = i32 & error = ()
    let receiver = rx.for_each(|num| {
        println!("{:?}", num);
        Ok(())
    }); 
    
    // Interval future has Item = () & Error = TimerError
    let timer = interval.for_each(|_| { // ForEach has Item = () &  Error = Interval::Error (TimerError);
        println!("interval");
        Ok(())
    }).then(|_| Ok(())); // Then has Item = Ok(())'s Item which is () & Ok(())'s Error =which is ()

    // Then's item and error are that of intofuture => Ok(()) returned by its closure  => |_| Ok(())
    // & Result<T, E> is an IntoFuture with Item = T & Error = E

    // in this case, then supresses the error from interval future. this future always suceeds

    let future = receiver.join(timer); // join expects error of 'timer' and 'receiver' be same

    thread::spawn(move || {
        for i in 0..10 {
            tx = tx.send(i).wait().unwrap();
            thread::sleep(Duration::new(1, 0));
        }
        thread::sleep(Duration::from_millis(10000));
    });


    let _ = reactor.run(future);
}