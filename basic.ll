; ModuleID = 'zoid_main'
source_filename = "zoid_main"
target triple = "x86_64-unknown-linux-gnu"

define i32 @main() {
entry:
  %var_decl = alloca i32, align 4
  store i32 42, ptr %var_decl, align 4
  %var_decl1 = alloca i32, align 4
  %var_expr = load i32, ptr %var_decl, align 4
  %binop = mul i32 %var_expr, 2
  %binop2 = add i32 %binop, 57
  store i32 %binop2, ptr %var_decl1, align 4
  %var_expr3 = load i32, ptr %var_decl1, align 4
  %binop4 = sub i32 %var_expr3, 57
  %binop5 = sdiv i32 %binop4, 2
  ret i32 %binop5
}