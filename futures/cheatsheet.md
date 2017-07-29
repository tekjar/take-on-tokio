```rust
// Constructing leaf futures
fn empty ()             -> Future<T, E>
fn ok    (T)            -> Future<T, E>
fn err   (E)            -> Future<T, E>
fn result(Result<T, E>) -> Future<T, E>

// General future constructor
fn poll_fn(FnMut(thread_local!(Task)) -> Poll<T, E>) -> Future<T, E>

// Mapping futures
fn Future::map     (Future<T, E>, FnOnce(T) -> U) -> Future<U, E>
fn Future::map_err (Future<T, E>, FnOnce(E) -> F) -> Future<T, F>
fn Future::from_err(Future<T, Into<E>>)           -> Future<T, E>

// Chaining (sequencing) futures
fn Future::then    (Future<T, E>, FnOnce(Result<T, E>) -> IntoFuture<U, F>) -> Future<U, F>
fn Future::and_then(Future<T, E>, FnOnce(T)            -> IntoFuture<U, E>) -> Future<U, E>
fn Future::or_else (Future<T, E>, FnOnce(E)            -> IntoFuture<T, F>) -> Future<T, F>
fn Future::flatten (Future<Future<T, E>, Into<E>>)                          -> Future<T, E>

// Joining (waiting) futures
fn Future::join (Future<T, E>, IntoFuture<U, E>)                                                       -> Future<(T, U),          E>
fn Future::join3(Future<T, E>, IntoFuture<U, E>, IntoFuture<V, E>)                                     -> Future<(T, U, V),       E>
fn Future::join4(Future<T, E>, IntoFuture<U, E>, IntoFuture<V, E>, IntoFuture<W, E>)                   -> Future<(T, U, V, W),    E>
fn Future::join5(Future<T, E>, IntoFuture<U, E>, IntoFuture<V, E>, IntoFuture<W, E>, IntoFuture<X, E>) -> Future<(T, U, V, W, X), E>
fn join_all     (IntoIterator<IntoFuture<T, E>>)                                                       -> Future<Vec<T>,          E>

// Selecting (racing) futures
fn Future::select (Future<T, E>, IntoFuture<T, E>) -> Future<(T, Future<T, E>), (E, Future<T, E>)>
fn Future::select2(Future<T, E>, IntoFuture<U, F>) -> Future<Either<(T, Future<U, F>), (U, Future<T, E>)>, Either<(E, Future<U, F>), (F, Future<T, E>)>>
fn select_all     (IntoIterator<IntoFuture<T, E>>) -> Future<(T, usize, Vec<Future<T, E>>), (E, usize, Vec<Future<T, E>>)>
fn select_ok      (IntoIterator<IntoFuture<T, E>>) -> Future<(T, Vec<Future<T, E>>), E>

// Utility
fn lazy         (FnOnce() -> IntoFuture<T, E>)             -> Future<T, E>
fn loop_fn      (S, FnMut(S) -> IntoFuture<Loop<T, S>, E>) -> Future<T, E>
fn Future::boxed(Future<T, E>+Send+'static)                -> Future<T, E>+Send+'static

// Miscellaneous
fn Future::into_stream   (Future<T, E>)            -> Stream<T, E>
fn Future::flatten_stream(Future<Stream<T, E>, E>) -> Stream<T, E>
fn Future::fuse          (Future<T, E>)            -> Future<T, E>
fn Future::catch_unwind  (Future<T, E>+UnwindSafe) -> Future<Result<T, E>, Any+Send>
fn Future::shared        (Future<T, E>)            -> Future<SharedItem<T>, SharedError<E>>+Clone
fn Future::wait          (Future<T, E>)            -> Result<T, E>
```