// #![feature(c_str_literals)]

use std::{
    collections::HashMap,
    ffi::{c_char, CString},
};

pub use llvm_sys as llvm;

use llvm::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction, LLVMVerifyModule},
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildAlloca, LLVMBuildBinOp,
        LLVMBuildLoad2, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildStore, LLVMConstIntOfString,
        LLVMConstRealOfString, LLVMContextCreate, LLVMCreateBuilderInContext,
        LLVMDoubleTypeInContext, LLVMDumpModule, LLVMFloatTypeInContext, LLVMFunctionType,
        LLVMGetTarget, LLVMInt128TypeInContext, LLVMInt16TypeInContext, LLVMInt32TypeInContext,
        LLVMInt64TypeInContext, LLVMInt8TypeInContext, LLVMModuleCreateWithNameInContext,
        LLVMPositionBuilderAtEnd, LLVMSetTarget, LLVMVoidTypeInContext,
    },
    prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef},
    target::{
        LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters,
        LLVM_InitializeAllDisassemblers, LLVM_InitializeAllTargetInfos,
        LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets,
    },
    target_machine::{
        LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetMachine, LLVMGetDefaultTargetTriple,
        LLVMGetTargetFromTriple, LLVMRelocMode, LLVMTargetRef,
    },
    transforms::pass_builder::{LLVMCreatePassBuilderOptions, LLVMRunPasses},
    LLVMOpcode,
};

use zoid_hlir::{
    HLIRBinaryOperator, HLIRExpression, HLIRFunction, HLIRLiteral, HLIRProgram, HLIRStatement,
    HLIRType,
};

#[derive(Debug, Clone)]
pub struct ZoidCodeGenContext<'source> {
    pub program: HLIRProgram<'source>,

    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,

    pub named_types: HashMap<&'source str, LLVMTypeRef>,
    pub named_values: HashMap<&'source str, LLVMValueRef>,
}

impl<'source> ZoidCodeGenContext<'source> {
    pub fn new(program: HLIRProgram<'source>) -> Self {
        let context = unsafe { LLVMContextCreate() };
        let module = unsafe { LLVMModuleCreateWithNameInContext(c"zoid_main".as_ptr(), context) };
        let builder = unsafe { LLVMCreateBuilderInContext(context) };

        unsafe { LLVMSetTarget(module, LLVMGetDefaultTargetTriple()) };

        let mut named_types = HashMap::new();

        {
            let mut add_type = |name: &'source str, ty: LLVMTypeRef| {
                named_types.insert(name, ty);
            };

            add_type("void", unsafe { LLVMVoidTypeInContext(context) });
            add_type("i8", unsafe { LLVMInt8TypeInContext(context) });
            add_type("i16", unsafe { LLVMInt16TypeInContext(context) });
            add_type("i32", unsafe { LLVMInt32TypeInContext(context) });
            add_type("i64", unsafe { LLVMInt64TypeInContext(context) });
            add_type("i128", unsafe { LLVMInt128TypeInContext(context) });
            add_type("u8", unsafe { LLVMInt8TypeInContext(context) });
            add_type("u16", unsafe { LLVMInt16TypeInContext(context) });
            add_type("u32", unsafe { LLVMInt32TypeInContext(context) });
            add_type("u64", unsafe { LLVMInt64TypeInContext(context) });
            add_type("u128", unsafe { LLVMInt128TypeInContext(context) });
            add_type("f32", unsafe { LLVMFloatTypeInContext(context) });
            add_type("f64", unsafe { LLVMDoubleTypeInContext(context) });
        }

        ZoidCodeGenContext {
            program,
            context,
            module,
            builder,
            named_types,
            named_values: HashMap::new(),
        }
    }

    pub fn verify(&self) {
        unsafe {
            let msg: *mut *mut c_char = std::ptr::null_mut();
            LLVMVerifyModule(
                self.module,
                LLVMVerifierFailureAction::LLVMPrintMessageAction,
                msg,
            )
        };
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.module);
        }
    }

    pub fn optimize(&mut self) {
        unsafe {
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllAsmPrinters();
            LLVM_InitializeAllAsmParsers();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllDisassemblers();
        };

        let triple = unsafe { LLVMGetTarget(self.module) };
        let target = unsafe {
            let mut t: LLVMTargetRef = std::ptr::null_mut();
            let err: *mut *mut c_char = std::ptr::null_mut();
            LLVMGetTargetFromTriple(triple, &mut t, err);
            t
        };

        let tm = unsafe {
            LLVMCreateTargetMachine(
                target,
                triple,
                c"generic".as_ptr(),
                c"".as_ptr(),
                LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
                LLVMRelocMode::LLVMRelocDefault,
                LLVMCodeModel::LLVMCodeModelDefault,
            )
        };

        let opts = unsafe { LLVMCreatePassBuilderOptions() };
        let passes = CString::new("constmerge,instcombine").unwrap();

        let _err = unsafe { LLVMRunPasses(self.module, passes.as_ptr(), tm, opts) };
    }

    pub fn codegen(&mut self) {
        for _global in &self.program.globals {
            todo!();
        }

        for (name, types) in &self.program.prototypes.clone() {
            self.codegen_prototype(&(name, types.clone()));
        }

        for function in &self.program.functions.clone() {
            self.codegen_function(function);
        }
    }

    fn codegen_type(&mut self, ty: &HLIRType) -> LLVMTypeRef {
        match ty {
            HLIRType::Void => self.named_types["void"],
            HLIRType::I8 => self.named_types["i8"],
            HLIRType::I16 => self.named_types["i16"],
            HLIRType::I32 => self.named_types["i32"],
            HLIRType::I64 => self.named_types["i64"],
            HLIRType::I128 => self.named_types["i128"],
            HLIRType::U8 => self.named_types["u8"],
            HLIRType::U16 => self.named_types["u16"],
            HLIRType::U32 => self.named_types["u32"],
            HLIRType::U64 => self.named_types["u64"],
            HLIRType::U128 => self.named_types["u128"],
            HLIRType::F32 => self.named_types["f32"],
            HLIRType::F64 => self.named_types["f64"],
            HLIRType::Var(_) => panic!("Type variable found in codegen"),
        }
    }

    fn codegen_prototype(&mut self, prototype: &(&'source str, (Vec<HLIRType>, HLIRType))) {
        let name = CString::new(prototype.0).unwrap();

        let mut param_types = Vec::new();
        for param_ty in &prototype.1 .0 {
            param_types.push(self.codegen_type(param_ty));
        }

        let ret_ty = self.codegen_type(&prototype.1 .1);

        let func_ty = unsafe {
            LLVMFunctionType(
                ret_ty,
                param_types.as_mut_ptr(),
                param_types.len() as u32,
                0,
            )
        };

        let func = unsafe { LLVMAddFunction(self.module, name.as_ptr(), func_ty) };

        self.named_values.insert(prototype.0, func);
    }

    fn codegen_function(&mut self, function: &HLIRFunction<'source>) {
        let func = *self.named_values.get(function.name).unwrap();

        let entry = unsafe { LLVMAppendBasicBlockInContext(self.context, func, c"entry".as_ptr()) };

        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, entry);
        }

        for statement in &function.body {
            self.codegen_statement(statement);
        }

        unsafe { LLVMVerifyFunction(func, LLVMVerifierFailureAction::LLVMPrintMessageAction) };
    }

    fn codegen_statement(&mut self, statement: &HLIRStatement<'source>) {
        match statement {
            HLIRStatement::VariableDeclaration { name, ty, value } => {
                let var = unsafe {
                    LLVMBuildAlloca(self.builder, self.codegen_type(ty), c"var_decl".as_ptr())
                };

                self.named_values.insert(name, var);

                let value = self.codegen_expression(value);

                unsafe {
                    LLVMBuildStore(self.builder, value, var);
                }
            }
            HLIRStatement::Return(value) => match value {
                Some(value) => {
                    let expr = self.codegen_expression(value);

                    unsafe { LLVMBuildRet(self.builder, expr) };
                }
                None => unsafe {
                    LLVMBuildRetVoid(self.builder);
                },
            },
        }
    }

    fn codegen_expression(&mut self, expression: &HLIRExpression<'source>) -> LLVMValueRef {
        match expression {
            HLIRExpression::Literal(literal, _) => self.codegen_literal(literal),
            HLIRExpression::Variable(name, ty) => {
                let ty = self.codegen_type(ty);
                let var = self.named_values.get(name).unwrap();
                unsafe { LLVMBuildLoad2(self.builder, ty, *var, c"var_expr".as_ptr()) }
            }
            HLIRExpression::BinaryOperation { lhs, op, rhs, ty } => {
                self.codegen_binary_operation(lhs, *op, rhs, ty)
            }
        }
    }

    fn codegen_literal(&mut self, literal: &HLIRLiteral<'source>) -> LLVMValueRef {
        match literal {
            HLIRLiteral::Integer(value, ty) => unsafe {
                let value = CString::new(*value).unwrap();
                LLVMConstIntOfString(self.codegen_type(ty), value.as_ptr(), 10)
            },
            HLIRLiteral::Float(value, ty) => unsafe {
                let value = CString::new(*value).unwrap();
                LLVMConstRealOfString(self.codegen_type(ty), value.as_ptr())
            },
        }
    }

    fn codegen_binary_operation(
        &mut self,
        lhs: &HLIRExpression<'source>,
        op: HLIRBinaryOperator,
        rhs: &HLIRExpression<'source>,
        ty: &HLIRType,
    ) -> LLVMValueRef {
        let lhs = self.codegen_expression(lhs);
        let rhs = self.codegen_expression(rhs);

        let is_signed = match ty {
            HLIRType::I8 | HLIRType::I16 | HLIRType::I32 | HLIRType::I64 | HLIRType::I128 => true,
            HLIRType::U8 | HLIRType::U16 | HLIRType::U32 | HLIRType::U64 | HLIRType::U128 => false,
            HLIRType::F32 | HLIRType::F64 => false,
            _ => panic!("Invalid type for binary operation"),
        };

        let is_float = match ty {
            HLIRType::F32 | HLIRType::F64 => true,
            _ => false,
        };

        let opcode = self.codegen_binary_operator(op, is_signed, is_float);

        unsafe { LLVMBuildBinOp(self.builder, opcode, lhs, rhs, c"binop".as_ptr()) }
    }

    fn codegen_binary_operator(
        &mut self,
        op: HLIRBinaryOperator,
        is_signed: bool,
        is_float: bool,
    ) -> LLVMOpcode {
        return if is_float {
            match op {
                HLIRBinaryOperator::Add => LLVMOpcode::LLVMFAdd,
                HLIRBinaryOperator::Sub => LLVMOpcode::LLVMFSub,
                HLIRBinaryOperator::Mul => LLVMOpcode::LLVMFMul,
                HLIRBinaryOperator::Div => LLVMOpcode::LLVMFDiv,
                _ => panic!("Invalid binary operator for floating point"),
            }
        } else {
            match op {
                HLIRBinaryOperator::Add => LLVMOpcode::LLVMAdd,
                HLIRBinaryOperator::Sub => LLVMOpcode::LLVMSub,
                HLIRBinaryOperator::Mul => LLVMOpcode::LLVMMul,
                HLIRBinaryOperator::Div => {
                    if is_signed {
                        LLVMOpcode::LLVMSDiv
                    } else {
                        LLVMOpcode::LLVMUDiv
                    }
                }
                HLIRBinaryOperator::Rem => {
                    if is_signed {
                        LLVMOpcode::LLVMSRem
                    } else {
                        LLVMOpcode::LLVMURem
                    }
                }
            }
        };
    }
}
