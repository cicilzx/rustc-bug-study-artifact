fn take_f(_: impl FnMut(&String)) {}

fn main() {
#[allow(unused_mut)] // `mut layer` needed to reproduce the diagnostic
take_f(|mut layer| {
layer.push('\n');
});
}