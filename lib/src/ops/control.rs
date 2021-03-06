use context::Context;
use value::{Value};
use super::{Ops, Op};

simple_op!(OpRepeat, ctx, (n: Double, f: Lambda), (), {
    for _ in 0..(n as usize) {
        f(ctx);
    }
});

simple_op!(OpMap, ctx, (f: Lambda, l: List), (), {
    ctx.enter_frame();
    for i in l {
        ctx.push(i);
        f(ctx);
    }
    ctx.exit_frame_list();
});

pub fn init(ops: &mut Ops) {
    ops.add(String::from("repeat"), OpRepeat::new);
    ops.add(String::from("map"), OpMap::new);
}