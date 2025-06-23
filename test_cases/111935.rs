// Lt indirection is necessary to make the lifetime of the function late-bound,
// in order to bypass some other bugs.
type Lt<'lt> = Option<*mut &'lt ()>;

// PASS!
#[cfg(case1)]
fn foo<'a>(_: Lt<'a>) -> impl Sized + 'a {
let _: () = foo(Lt::<'static>::None);
// invalid defining use: Opaque<'static> := ()
}

// ICE!
#[cfg(case2)]
fn foo<'a>(_: Lt<'a>) -> impl Sized + 'a {
let _: () = foo(Lt::<'_>::None);
// invalid defining use: Opaque<'_> := ()
}

// PASS!
#[cfg(case3)]
fn foo<'a, 'b>(_: Lt<'a>, _: Lt<'b>) -> impl Sized + 'a + 'b {
let _: () = foo(Lt::<'a>::None, Lt::<'a>::None);
// invalid defining use: Opaque<'a, 'a> := ()
// because of the use of equal lifetimes in args
}

fn main() {}