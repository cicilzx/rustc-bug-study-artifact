trait Service {
type S;
}

trait Framing {
type F;
}

impl Framing for () {
type F = ();
}

trait HttpService: Service<S = <() as Framing>::F> {}

fn main() {
let _: dyn HttpService<S = ()>;
}