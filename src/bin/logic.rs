use z3::{ast::Bool, Config, Context, Solver};

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let p = Bool::new_const(&ctx, "p");

    // true theorem
    {
        let my_true_thm = p.implies(&p);

        println!("my_true_thm: {}", my_true_thm);

        println!("proving my_true_thm...");
        prove(&ctx, &my_true_thm);
    }

    // false theorem
    {
        let q = Bool::new_const(&ctx, "q");

        let my_false_thm = q.implies(&p);
        println!("my_false_thm: {}", my_false_thm);
        println!("proving my_false_thm...");
        prove(&ctx, &my_false_thm);
    }
}

fn prove(ctx: &Context, proposition: &Bool) {
    let solver = Solver::new(&ctx);
    solver.assert(&proposition.not());
    let sat = solver.check();
    match sat {
        z3::SatResult::Unsat => {
            println!("proved");
        }
        _ => {
            println!("failed to prove");
        }
    }
}
