trait StateTrait {
fn handle_state(&self);
}

trait StateView: Sized {
fn view(_: &StateTrait) -> Vec<Self>;
}

fn main() {}