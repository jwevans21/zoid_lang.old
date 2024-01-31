use std::collections::HashMap;

use zoid_ast::{
    BinaryOperator, Expression, Literal, Parameter, Program, Statement, TopLevelExpression, Type,
};
use zoid_hlir::{
    HLIRBinaryOperator, HLIRExpression, HLIRFunction, HLIRLiteral, HLIRParameter, HLIRProgram,
    HLIRStatement, HLIRType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint<'source> {
    Equal(HLIRType, HLIRType),
    Binding(&'source str, HLIRType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoidLoweringContext<'source> {
    pub program: Program<'source>,
    pub hlir_program: HLIRProgram<'source>,
    pub substitution_map: HashMap<usize, HLIRType>,
    pub constraints: Vec<Constraint<'source>>,
    pub next_variable_id: usize,
}

impl<'source> ZoidLoweringContext<'source> {
    pub fn new(program: Program<'source>) -> Self {
        Self {
            program,
            hlir_program: HLIRProgram {
                globals: HashMap::new(),
                prototypes: HashMap::new(),
                functions: Vec::new(),
            },
            substitution_map: HashMap::new(),
            constraints: Vec::new(),
            next_variable_id: 0,
        }
    }

    pub fn lower(&mut self) -> HLIRProgram<'source> {
        for top_level in self.program.0.clone().iter() {
            self.lower_top_level_expression(top_level);
        }

        self.solve_constraints()
    }

    fn solve_constraints(&mut self) -> HLIRProgram<'source> {
        for constraint in self.constraints.clone().iter() {
            match constraint {
                Constraint::Equal(ty1, ty2) => {
                    self.unify(&ty1, &ty2);
                }
                _ => (), // Constraint::Binding(name, ty) => {
                         //     self.substitution_map.insert(ty.var_id().unwrap(), *ty);
                         // }
            }
        }
        self.apply_substitutions();

        self.constraints.clear();

        self.propagate_types()
    }

    fn unify(&mut self, ty1: &HLIRType, ty2: &HLIRType) {
        let ty1 = self.apply_substitution(ty1);
        let ty2 = self.apply_substitution(ty2);

        match (ty1, ty2) {
            (HLIRType::Var(id1), HLIRType::Var(id2)) => {
                if id1 == id2 {
                    return;
                }
                self.substitution_map.insert(id1, HLIRType::Var(id2));
            }
            (HLIRType::Var(id), ty) | (ty, HLIRType::Var(id)) => {
                self.substitution_map.insert(id, ty);
            }
            // (HLIRType::Function(params1, ret1), HLIRType::Function(params2, ret2)) => {
            //     if params1.len() != params2.len() {
            //         panic!("Function types have different number of parameters");
            //     }
            //     for (p1, p2) in params1.iter().zip(params2.iter()) {
            //         self.unify(p1, p2);
            //     }
            //     self.unify(ret1, ret2);
            // }
            (ty1, ty2) => {
                if ty1 != ty2 {
                    panic!("Cannot unify types {:?} and {:?}", ty1, ty2);
                }
            }
        }
    }

    fn apply_substitution(&self, ty: &HLIRType) -> HLIRType {
        match ty {
            HLIRType::Var(id) => {
                if let Some(substituted_ty) = self.substitution_map.get(id) {
                    self.apply_substitution(substituted_ty)
                } else {
                    *ty
                }
            }
            // HLIRType::Function(params, ret) => HLIRType::Function(
            //     params.iter().map(|p| self.apply_substitution(p)).collect(),
            //     Box::new(self.apply_substitution(ret)),
            // ),
            _ => *ty,
        }
    }

    fn apply_substitutions(&mut self) {
        for (id, ty) in self.substitution_map.clone().iter() {
            self.substitution_map
                .insert(*id, self.apply_substitution(ty));
        }
    }

    fn lower_top_level_expression(&mut self, top_level: &TopLevelExpression<'source>) {
        match top_level {
            TopLevelExpression::Function {
                name,
                parameters,
                return_type,
                body,
            } => {
                let mut hlir_parameters = Vec::new();
                for parameter in parameters {
                    hlir_parameters.push(self.lower_parameter(parameter));
                }
                let hlir_return_type = match return_type {
                    Some(ty) => self.lower_type(ty),
                    None => HLIRType::Void,
                };

                let hlir_prototype = (
                    hlir_parameters.iter().map(|p| p.ty).collect(),
                    hlir_return_type,
                );
                self.hlir_program.prototypes.insert(name, hlir_prototype);

                let mut named_values = HashMap::new();

                let mut hlir_body = Vec::new();
                for statement in body {
                    hlir_body.push(self.lower_statement(
                        statement,
                        &mut named_values,
                        &hlir_return_type,
                    ));
                }

                self.hlir_program.functions.push(HLIRFunction {
                    name,
                    parameters: hlir_parameters,
                    return_type: hlir_return_type,
                    body: hlir_body,
                });
            }
        }
    }

    fn lower_parameter(&mut self, parameter: &Parameter<'source>) -> HLIRParameter<'source> {
        HLIRParameter {
            name: parameter.name,
            ty: self.lower_type(&parameter.ty),
        }
    }

    fn lower_type(&mut self, ty: &Type) -> HLIRType {
        match ty {
            Type::I8 => HLIRType::I8,
            Type::I16 => HLIRType::I16,
            Type::I32 => HLIRType::I32,
            Type::I64 => HLIRType::I64,
            Type::I128 => HLIRType::I128,
            Type::U8 => HLIRType::U8,
            Type::U16 => HLIRType::U16,
            Type::U32 => HLIRType::U32,
            Type::U64 => HLIRType::U64,
            Type::U128 => HLIRType::U128,
            Type::F32 => HLIRType::F32,
            Type::F64 => HLIRType::F64,
            Type::Void => HLIRType::Void,
        }
    }

    fn lower_statement(
        &mut self,
        statement: &Statement<'source>,
        named_values: &mut HashMap<&'source str, HLIRType>,
        return_type: &HLIRType,
    ) -> HLIRStatement<'source> {
        match statement {
            Statement::VariableDeclaration { name, ty, value } => {
                let hlir_ty = match ty {
                    Some(ty) => self.lower_type(ty),
                    None => {
                        let id = self.next_variable_id;
                        self.next_variable_id += 1;
                        HLIRType::Var(id)
                    }
                };
                let hlir_value = self.lower_expression(value, named_values);
                self.constraints
                    .push(Constraint::Equal(hlir_ty, hlir_value.ty()));
                named_values.insert(name, hlir_ty);
                HLIRStatement::VariableDeclaration {
                    name,
                    ty: hlir_ty,
                    value: hlir_value,
                }
            }
            Statement::Return(ref value) => match value {
                Some(value) => {
                    let hlir_value = self.lower_expression(value, named_values);

                    self.constraints
                        .push(Constraint::Equal(hlir_value.ty(), *return_type));

                    HLIRStatement::Return(Some(hlir_value))
                }
                None => {
                    if *return_type == HLIRType::Void {
                        HLIRStatement::Return(None)
                    } else {
                        panic!("Return type does not match function return type");
                    }
                }
            },
        }
    }

    fn lower_expression(
        &mut self,
        expression: &Expression<'source>,
        named_values: &HashMap<&'source str, HLIRType>,
    ) -> HLIRExpression<'source> {
        match expression {
            Expression::Literal(literal) => self.lower_literal(literal),
            Expression::Variable(name) => {
                let ty = named_values.get(name).unwrap();
                HLIRExpression::Variable(name, *ty)
            }
            Expression::BinaryOperation {
                ref lhs,
                op,
                ref rhs,
            } => {
                let hlir_lhs = Box::new(self.lower_expression(lhs, named_values));
                let hlir_rhs = Box::new(self.lower_expression(rhs, named_values));
                let ty = self.lower_binary_operator_res_ty(*op, &hlir_lhs.ty(), &hlir_rhs.ty());
                HLIRExpression::BinaryOperation {
                    lhs: hlir_lhs,
                    op: self.lower_binary_operator(*op),
                    rhs: hlir_rhs,
                    ty,
                }
            }
        }
    }

    fn lower_literal(&mut self, literal: &Literal<'source>) -> HLIRExpression<'source> {
        match literal {
            Literal::Integer(value) => {
                let id = self.next_variable_id;
                self.next_variable_id += 1;
                let ty = HLIRType::Var(id);
                HLIRExpression::Literal(HLIRLiteral::Integer(value, ty), ty)
            }
            Literal::Float(value) => {
                let id = self.next_variable_id;
                self.next_variable_id += 1;
                let ty = HLIRType::Var(id);
                HLIRExpression::Literal(HLIRLiteral::Float(value, ty), ty)
            }
        }
    }

    fn lower_binary_operator(&mut self, op: BinaryOperator) -> HLIRBinaryOperator {
        match op {
            BinaryOperator::Add => HLIRBinaryOperator::Add,
            BinaryOperator::Sub => HLIRBinaryOperator::Sub,
            BinaryOperator::Mul => HLIRBinaryOperator::Mul,
            BinaryOperator::Div => HLIRBinaryOperator::Div,
            BinaryOperator::Rem => HLIRBinaryOperator::Rem,
        }
    }

    fn lower_binary_operator_res_ty(
        &mut self,
        _op: BinaryOperator,
        lhs_ty: &HLIRType,
        rhs_ty: &HLIRType,
    ) -> HLIRType {
        let id = self.next_variable_id;
        self.next_variable_id += 1;
        let ty = HLIRType::Var(id);
        self.constraints.push(Constraint::Equal(*lhs_ty, *rhs_ty));
        self.constraints.push(Constraint::Equal(*lhs_ty, ty));
        self.constraints.push(Constraint::Equal(*rhs_ty, ty));

        ty
    }

    fn propagate_types(&mut self) -> HLIRProgram<'source> {
        let mut new_program = HLIRProgram {
            globals: self.clone().hlir_program.globals,
            prototypes: self.clone().hlir_program.prototypes,
            functions: Vec::new(),
        };

        for function in self.clone().hlir_program.functions.iter_mut() {
            new_program
                .functions
                .push(self.propagate_types_in_function(function));
        }

        new_program
    }

    fn propagate_types_in_function(
        &mut self,
        function: &mut HLIRFunction<'source>,
    ) -> HLIRFunction<'source> {
        let mut named_values = HashMap::new();
        for parameter in function.parameters.iter() {
            named_values.insert(parameter.name, parameter.ty);
        }

        let mut new_body = Vec::new();
        for statement in function.body.iter_mut() {
            new_body.push(self.propagate_types_in_statement(statement, &mut named_values));
        }

        HLIRFunction {
            name: function.name,
            parameters: function.parameters.clone(),
            return_type: function.return_type,
            body: new_body,
        }
    }

    fn propagate_types_in_statement(
        &mut self,
        statement: &mut HLIRStatement<'source>,
        named_values: &mut HashMap<&'source str, HLIRType>,
    ) -> HLIRStatement<'source> {
        match statement {
            HLIRStatement::VariableDeclaration { name, ty, value } => {
                *ty = self.apply_substitution(ty);
                named_values.insert(name, *ty);

                HLIRStatement::VariableDeclaration {
                    name,
                    ty: *ty,
                    value: self.propagate_types_in_expression(value, named_values),
                }
            }
            HLIRStatement::Return(Some(value)) => HLIRStatement::Return(Some(
                self.propagate_types_in_expression(value, named_values),
            )),
            HLIRStatement::Return(None) => HLIRStatement::Return(None),
        }
    }

    fn propagate_types_in_expression(
        &mut self,
        expression: &mut HLIRExpression<'source>,
        named_values: &mut HashMap<&'source str, HLIRType>,
    ) -> HLIRExpression<'source> {
        match expression {
            HLIRExpression::Variable(name, ty) => {
                if let Some(t) = named_values.get(name) {
                    *ty = *t;
                } else {
                    *ty = self.apply_substitution(ty);
                }

                HLIRExpression::Variable(name, *ty)
            }
            HLIRExpression::BinaryOperation { lhs, rhs, ty, op } => {
                let lhs = self.propagate_types_in_expression(lhs, named_values);
                let rhs = self.propagate_types_in_expression(rhs, named_values);
                *ty = self.apply_substitution(ty);

                HLIRExpression::BinaryOperation {
                    lhs: Box::new(lhs),
                    op: op.clone(),
                    rhs: Box::new(rhs),
                    ty: *ty,
                }
            }
            HLIRExpression::Literal(literal, ty) => {
                *ty = self.apply_substitution(ty);
                HLIRExpression::Literal(self.propagate_types_in_literal(literal), *ty)
            }
        }
    }

    fn propagate_types_in_literal(
        &mut self,
        literal: &HLIRLiteral<'source>,
    ) -> HLIRLiteral<'source> {
        match literal {
            HLIRLiteral::Integer(value, ty) => {
                HLIRLiteral::Integer(*value, self.apply_substitution(ty))
            }
            HLIRLiteral::Float(value, ty) => {
                HLIRLiteral::Float(*value, self.apply_substitution(ty))
            }
        }
    }
}
