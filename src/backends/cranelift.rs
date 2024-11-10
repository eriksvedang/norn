#![allow(dead_code)]

use crate::backend::Backend;
use codegen::ir::UserFuncName;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use isa::TargetIsa;
use std::{mem, sync::Arc};

pub struct CraneliftBackend {
    module: JITModule,
}

impl CraneliftBackend {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        // FIXME set back to true once the x64 backend supports it.
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {msg}");
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let mut jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        //jit_builder.symbol("foo", foo as *const u8);

        let mut module = JITModule::new(jit_builder);

        Self { module }
    }
}

//         let callee = self
//             .module
//             .declare_function(&name, Linkage::Import, &sig)
//             .expect("problem declaring function");
//         let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

impl Backend for CraneliftBackend {
    fn define_function(&mut self, name: &str) {
        let mut ctx = self.module.make_context();
        let mut func_ctx = FunctionBuilderContext::new();

        let mut sig_a = self.module.make_signature();
        sig_a.params.push(AbiParam::new(types::I32));
        sig_a.returns.push(AbiParam::new(types::I32));

        let mut sig_b = self.module.make_signature();
        sig_b.returns.push(AbiParam::new(types::I32));

        let func_a = self
            .module
            .declare_function("a", Linkage::Local, &sig_a)
            .unwrap();
        let func_b = self
            .module
            .declare_function("b", Linkage::Local, &sig_b)
            .unwrap();

        ctx.func.signature = sig_a;
        ctx.func.name = UserFuncName::user(0, func_a.as_u32());

        {
            let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            let block = bcx.create_block();

            bcx.switch_to_block(block);
            bcx.append_block_params_for_function_params(block);
            let param = bcx.block_params(block)[0];
            let cst = bcx.ins().iconst(types::I32, 111);
            let add = bcx.ins().iadd(cst, param);
            bcx.ins().return_(&[add]);
            bcx.seal_all_blocks();
            bcx.finalize();
        }
        self.module.define_function(func_a, &mut ctx).unwrap();
        self.module.clear_context(&mut ctx);

        let mut foo_sig = self.module.make_signature();
        foo_sig.params.push(AbiParam::new(types::I32));
        foo_sig.returns.push(AbiParam::new(types::I32));
        let callee = self
            .module
            .declare_function("foo", Linkage::Import, &foo_sig)
            .unwrap();

        ctx.func.signature = sig_b;
        ctx.func.name = UserFuncName::user(0, func_b.as_u32());

        {
            let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            let block = bcx.create_block();

            bcx.switch_to_block(block);
            let local_func = self.module.declare_func_in_func(func_a, &mut bcx.func);
            //let local_func = self.module.declare_func_in_func(callee, &mut bcx.func);
            let x: i64 = 666;
            let arg = bcx.ins().iconst(types::I32, x);
            let call = bcx.ins().call(local_func, &[arg]);
            let value = {
                let results = bcx.inst_results(call);
                assert_eq!(results.len(), 1);
                results[0]
            };
            bcx.ins().return_(&[value]);
            bcx.seal_all_blocks();
            bcx.finalize();
        }
        self.module.define_function(func_b, &mut ctx).unwrap();
        self.module.clear_context(&mut ctx);

        // Perform linking.
        self.module.finalize_definitions().unwrap();

        // Get a raw pointer to the generated code.
        let code_b = self.module.get_finalized_function(func_b);

        // Cast it to a rust function pointer type.
        let ptr_b = unsafe { mem::transmute::<_, extern "C" fn() -> u32>(code_b) };

        // Call it!
        let res = ptr_b();

        println!("res = {}", res);
    }

    // fn register_c_function(&mut self) {
    //     self.module
    //     jit_builder.symbol("foo", foo as *const u8);
    // }
}
