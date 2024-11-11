#![allow(dead_code)]

use crate::{ast::AstNode, backend::Backend, ty::Ty};
use codegen::ir::UserFuncName;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use std::{collections::HashMap, mem};

pub struct CraneliftBackend {
    module: JITModule,
}

impl CraneliftBackend {
    pub fn new(c_functions: HashMap<String, *const u8>) -> Self {
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

        for (c_function_name, c_function_ptr) in c_functions {
            jit_builder.symbol(c_function_name, c_function_ptr);
        }

        let module = JITModule::new(jit_builder);

        Self { module }
    }

    fn visit_ast_node<'a>(
        &'a mut self,
        fb: &'a mut FunctionBuilder,
        block: Block,
        node: AstNode,
    ) -> Value {
        // let param = fb.block_params(block)[0];
        // let cst = fb.ins().iconst(types::I32, 37);
        // let add = fb.ins().iadd(cst, param);
        // add

        match node {
            AstNode::Do(statements) => {
                let mut res: Value = Value::from_u32(0); // FIXME
                for s in statements {
                    res = self.visit_ast_node(fb, block, s);
                }
                res
            }
            AstNode::Call(path, args) => {
                let mut sig = self.module.make_signature();
                sig.params.push(AbiParam::new(types::I32));
                sig.returns.push(AbiParam::new(types::I32));
                let callee = self
                    .module
                    .declare_function(&path.to_string(), Linkage::Import, &sig)
                    .unwrap();
                let local_func = self.module.declare_func_in_func(callee, &mut fb.func);
                let i_arg: i64 = match args[0] {
                    AstNode::Constant(x) => x as i64,
                    _ => panic!(),
                };
                let arg = fb.ins().iconst(types::I32, i_arg);
                let call = fb.ins().call(local_func, &[arg]);
                let value = {
                    let results = fb.inst_results(call);
                    assert_eq!(results.len(), 1);
                    results[0]
                };
                value
            }
            AstNode::SetLocal(_, _) => todo!(),
            AstNode::Variable(_) => todo!(),
            AstNode::Constant(_) => todo!(),
        }
    }
}

impl Backend for CraneliftBackend {
    type FunctionHandle = FuncId;

    fn define_function(
        &mut self,
        name: &str,
        _func_ty: Ty,
        _body: AstNode,
    ) -> Self::FunctionHandle {
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(types::I32));
        sig.returns.push(AbiParam::new(types::I32));

        let func = self
            .module
            .declare_function(name, Linkage::Local, &sig)
            .unwrap();

        let mut ctx = self.module.make_context();
        ctx.func.signature = sig;
        ctx.func.name = UserFuncName::user(0, func.as_u32());

        let mut fb_ctx = FunctionBuilderContext::new();
        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut fb_ctx);
        let block = fb.create_block();
        fb.switch_to_block(block);
        fb.append_block_params_for_function_params(block);
        let ret = self.visit_ast_node(&mut fb, block, _body);
        fb.ins().return_(&[ret]);
        fb.seal_all_blocks();
        fb.finalize();

        self.module.define_function(func, &mut ctx).unwrap();
        self.module.clear_context(&mut ctx);

        self.module.finalize_definitions().unwrap();

        func
    }

    fn call_function(&self, function_handle: &Self::FunctionHandle) -> i32 {
        let code = self.module.get_finalized_function(*function_handle);
        let ptr = unsafe { mem::transmute::<_, extern "C" fn() -> i32>(code) };
        ptr()
    }

    fn call_function_one_arg<T>(&self, function_handle: &Self::FunctionHandle, arg0: T) -> i32 {
        let code = self.module.get_finalized_function(*function_handle);
        let ptr = unsafe { mem::transmute::<_, extern "C" fn(T) -> i32>(code) };
        ptr(arg0)
    }
}
