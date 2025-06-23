pub async fn run(permit: ActionPermit<'_, ()>, ctx: &mut core::task::Context<'_>) {
run2(permit, ctx);
}

fn run2<T>(permit: ActionPermit<'_, T>, ctx: &mut core::task::Context) {
_ = || {
let mut fut = ActionPermit::perform(permit);
let fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
_ = core::future::Future::poll(fut, ctx);
};
}

pub struct ActionPermit<'a, T> {
_guard: core::cell::Ref<'a, T>,
}

impl<'a, T> ActionPermit<'a, T> {
async fn perform(self) {
core::future::ready(()).await
}
}

fn main() {}