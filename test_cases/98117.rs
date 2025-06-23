trait Outlives<'a>: 'a {}
impl<'a, T> Outlives<'a> for &'a T {}

fn step2<T>(t: T) -> &'static str
where
&'static T: Outlives<'static>,
T: AsRef<str>,
{
AsRef::as_ref(Box::leak(Box::new(t) as Box<dyn AsRef<str> + 'static>))
}

fn step1<T>(t: T) -> &'static str
where
for<'a> &'a T: Outlives<'a>,
T: AsRef<str>,
{
step2(t)
}

fn main() {
let s: &'static str = step1(&String::from("blah blah blah"));
println!("{s}");
}