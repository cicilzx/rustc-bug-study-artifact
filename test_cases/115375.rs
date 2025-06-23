type Lt<'lt> = *mut &'lt u8;

fn outlives<'lt, T: 'lt>(_: T, _: Lt<'lt>) {}

pub fn test<A>(arg: A, lt: Lt<'_>, _: &str) {
outlives(arg, lt);
//~^ ERROR the parameter type `A` may not live long enough
}

fn main() {}