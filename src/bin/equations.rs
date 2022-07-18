use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let two = Int::from_i64(&ctx, 2);
    let ten = Int::from_i64(&ctx, 10);
    let seven = Int::from_i64(&ctx, 7);

    let solver = Solver::new(&ctx);

    // assertions
    {
        solver.assert(&x.gt(&two)); // x > 2
        solver.assert(&y.lt(&ten)); // y < 10

        let product = Int::mul(&ctx, &vec![&two, &y]);
        let sum = Int::add(&ctx, &vec![&x, &product]);
        solver.assert(&sum._eq(&seven)); // x + 2*y == 7
    }

    let sat = solver.check();
    assert_eq!(sat, SatResult::Sat);

    let model = solver.get_model().unwrap();
    println!("{}", model);
    let xv = model.eval(&x, true).unwrap().as_i64().unwrap();
    let yv = model.eval(&y, true).unwrap().as_i64().unwrap();
    assert_eq!(xv, 7);
    assert_eq!(yv, 0);
}
