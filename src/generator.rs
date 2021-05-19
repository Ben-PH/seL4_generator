extern crate pest_derive;
use codegen::{Scope, Function};
use super::parser::Field;

pub fn gen(parsed_blocks: Vec<super::parser::Block>) -> Scope {
    let mut scope = Scope::new();

    for block in parsed_blocks {
        // pub struct {block-name} {
        //     inner: u64,
        // }
        scope.new_struct(&block.name)
            .field("inner", "u64");

        // accumulate the bit widths
        let mut acc = 64;

        // impl {block-name} {
        //     #[inline(always)]
        //     pub fn new() -> Self {
        //         Self{inner: 0}
        //     }
        //     ...

        let imp = scope.new_impl(&block.name);
        let mut new_fn = Function::new("new");
        new_fn.attr("inline(always)");
        new_fn.ret("Self");
        new_fn.line("Self{inner: 0}");
        imp.push_fn(new_fn);

        // add getter and setter functions for each field
        for field in block.fields {
            match field {
                Field::Val(name, force) => {

                    acc -= force;
                    imp.push_fn(gen_getter(acc, &name, force));
                    imp.push_fn(gen_setter(acc, &name, force));

                }
                Field::Padding(force) => acc -= force,
            }
        }
    }
    scope
}

fn gen_setter(acc: u32, name: &String, force: u32) -> Function {
    let ones: u64 = !0;
    let mask = ones >> (64 - force);
    let mut setter = Function::new(&format!("set_{}", name));

    // TODO: check if bit-widths match standard widths (1, 8, 16, etc...)
    //       and set the argument types, and ommit the value check
    // TODO: return Err(s, {upper-bound}) for the error
    //
    // pub fn set_{field_name}(self, {field_name}: usize) -> <Self, (Self,())> {...}
    setter.attr("inline(always)");
    setter.ret("<Self, (Self, ())>");
    setter.arg_self();
    setter.arg(name, "usize");

    // value being set fits within the bit-width, else return Error
    setter.line(format!("if {} > 0x{:x} {{ return Err(s, ()); }}", &name, mask));

    // limitation of generator library: Can't take mutable ownership of self,
    // so we need to force mutability
    setter.line("let mut s = self;");

    // do the value-setting
    setter.line(&format!("s.inner &= !0x{:x};", mask << acc));
    setter.line(&format!("s.inner |= (({} as u64) << {}) & 0x{:x};", name, acc, mask << acc));
    setter.line("Ok(s)");

    setter
}

fn gen_getter(acc: u32, name: &String, force: u32) -> Function {
    let ones: u64 = !0;
    let mask = ones >> (64 - force);

    // pub fn {field-name}(&self) -> u64 {...}
    let mut getter = Function::new(&format!("{}", name));
    getter.attr("inline(always)");
    getter.ret("u64");
    getter.arg_ref_self();

    // mask and shift to get value
    getter.line(&format!("(self.inner & 0x{:x}) >> {}", mask << acc, acc));
    getter
}
