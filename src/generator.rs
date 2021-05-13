extern crate pest_derive;
use codegen::{Scope, Function};
use super::parser::Field;

pub fn gen(parsed: Vec<super::parser::Generated>) -> Scope {
    let mut scope = Scope::new();

    for g in parsed {
        scope.new_struct(&g.name)
            .field("inner", "u64");

        let mut acc = 64;
        let i = scope.new_impl(&g.name);
        let mut new_fn = Function::new("new");
        new_fn.attr("inline(always)");
        new_fn.ret("Self");
        new_fn.line("let inner = 0");

        for fld in g.fields {
            match fld {
                Field::Val(name, force) => {

                    acc -= force;
                    gen_new(acc, &mut new_fn, &name, force);
                    i.push_fn(gen_getter(acc, &name, force));
                    i.push_fn(gen_setter(acc, &name, force));

                }
                Field::Padding(force) => acc -= force,
            }
        }
        new_fn.line("Self{inner}");
        i.push_fn(new_fn);
    }
    scope
}

fn gen_setter(acc: u32, name: &String, force: u32) -> Function {
    let ones: u64 = !0;
    let mask = ones >> (64 - force);
    let mut setter = Function::new(&format!("set_{}", name));
    setter.attr("inline(always)");
    setter.ret("Self");
    setter.arg_self();
    setter.arg(name, "u32");
    setter.line("let mut s = self;");
    setter.line(&format!("// reset bits to 0"));
    setter.line(&format!("s.inner &= !0x{:x};", mask << acc));
    setter.line(&format!("s.inner |= (({} as u64) << {}) & 0x{:x};", name, acc, mask << acc));
    setter.line("s");
    //seL4_MessageInfo.words[0] |= (v64 << 9) & 0xe00ull;
    setter
}
fn gen_getter(acc: u32, name: &String, force: u32) -> Function {
    let ones: u64 = !0;
    let mask = ones >> (64 - force);
    let mut getter = Function::new(&format!("{}", name));
    getter.attr("inline(always)");
    getter.ret("u64");
    getter.arg_ref_self();
    getter.line(&format!("(self.inner & 0x{:x}) >> {}", mask << acc, acc));
    getter
}
fn gen_new(acc: u32, new_fn: &mut Function, name: &String, force: u32) {
    let ones: u64 = !0;
    let mask = ones >> (64 - force);
    new_fn.arg(&name, "u64");
    let term = match acc {
        0 => ";",
        _ => "",
    };
    new_fn.line(format!("    | ({} & 0x{:x}) << {}{}", name, mask, acc, term));
}
