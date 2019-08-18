///任意の式を表す
#[derive(Debug, PartialEq)]
pub enum Expr {
    ConstantVal(ConstantVal),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    ///式を評価する
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstantVal(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval()
        }
    }
}

///定数を表す
#[derive(Debug, PartialEq)]
pub struct ConstantVal(i32);

impl ConstantVal {
    ///ConstantValを生成する
    pub fn new(val: i32) -> ConstantVal {
        ConstantVal(val)
    }

    ///定数を評価する
    pub fn eval(&self) -> i32 {
        self.0
    }
}

#[test]
fn constant_val_test() {
    let source = 55;
    let constant_val = ConstantVal::new(source);
    assert_eq!(
        constant_val.eval(),
        source
    );
}

///演算子種別
#[derive(Debug, PartialEq)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

///二項演算子を表す
#[derive(Debug, PartialEq)]
pub struct BinaryOp {
    //適応する演算子種別
    op_kind: OpKind,
    //演算子の左にある式
    left_expr: Expr,
    //演算子の右にある式
    right_expr: Expr,
}

impl BinaryOp {
    ///BinaryOpを生成する
    pub fn new(op_kind: OpKind, left_expr: Expr, right_expr: Expr) -> BinaryOp {
        BinaryOp { op_kind, left_expr, right_expr }
    }

    ///二項演算式を評価する
    pub fn eval(&self) -> i32 {
        let right = self.right_expr.eval();
        let left = self.left_expr.eval();
        match self.op_kind {
            OpKind::Add => left + right,
            OpKind::Sub => left - right,
            OpKind::Mul => left * right,
            OpKind::Div => left / right
        }
    }
}

#[test]
fn binary_op_test() {
    //13*(5+1)の式を生成
    let binary_op = BinaryOp::new(
        OpKind::Mul,
        Expr::ConstantVal(ConstantVal::new(13)),
        Expr::BinaryOp(
            Box::new(
                BinaryOp::new(
                    OpKind::Add,
                    Expr::ConstantVal(ConstantVal::new(5)),
                    Expr::ConstantVal(ConstantVal::new(1)),
                )
            )
        ),
    );
    let expect = 13 * (5 + 1);
    assert_eq!(
        binary_op.eval(),
        expect
    );
}