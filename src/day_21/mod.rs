use core::panic;

use nom::character::complete::alpha1;
use nom::character::complete::{char, line_ending, *};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use rustc_hash::FxHashMap;

pub type Id = u16;
pub type Val = i16;
pub type LargeVal = i64;

#[derive(Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn eval(&self, left: LargeVal, right: LargeVal) -> LargeVal {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }

    fn inverse(&self) -> Operation {
        match self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Mul => Operation::Div,
            Operation::Div => Operation::Mul,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Job {
    Number(Val),
    Computation(Id, Operation, Id),
}

#[derive(Clone)]
pub struct Monkey {
    id: Id,
    job: Job,
    result: Option<LargeVal>,
}

#[derive(Clone)]
pub struct Input {
    monkeys: Vec<Monkey>,
    root_id: Id,
    my_id: Id,
}

struct MonkeyParser<'a, 'b> {
    next_id: Id,
    id_map: &'b mut FxHashMap<&'a str, Id>,
}

impl<'a, 'b> MonkeyParser<'a, 'b> {
    fn new(id_map: &'b mut FxHashMap<&'a str, Id>) -> Self {
        Self { next_id: 0, id_map }
    }

    fn get_id(&mut self, name: &'a str) -> Id {
        *self.id_map.entry(name).or_insert_with(|| {
            let out = self.next_id;
            self.next_id += 1;
            out
        })
    }

    fn parse_job(&mut self, input: &'a str) -> nom::IResult<&'a str, Job> {
        let (input, num) = opt(delimited(space0, i16, space0))(input)?;
        if let Some(n) = num {
            return Ok((input, Job::Number(n)));
        }

        let (input, word1) = delimited(space0, alpha1, space0)(input)?;
        let (input, (op, _, word2)) = tuple((one_of("+/-*"), space0, alpha1))(input)?;
        let id1 = self.get_id(word1);
        let id2 = self.get_id(word2);
        let op = match op {
            '*' => Operation::Mul,
            '/' => Operation::Div,
            '+' => Operation::Add,
            '-' => Operation::Sub,
            _ => unreachable!(),
        };

        Ok((input, Job::Computation(id1, op, id2)))
    }
}

impl<'a, 'b> nom::Parser<&'a str, Monkey, nom::error::Error<&'a str>> for MonkeyParser<'a, 'b> {
    fn parse(
        &mut self,
        input: &'a str,
    ) -> nom::IResult<&'a str, Monkey, nom::error::Error<&'a str>> {
        let (input, (name, _, _, rest)) =
            tuple((alpha1, char(':'), space0, not_line_ending))(input)?;
        let (_, job) = self.parse_job(rest)?;
        let id = self.get_id(name);

        Ok((
            input,
            Monkey {
                id,
                job,
                result: None,
            },
        ))
    }
}

#[aoc_generator(day21)]
pub fn input_generator(data: &str) -> Input {
    let mut id_map = FxHashMap::default();

    let (unparsed_input, mut monkeys) =
        separated_list1(line_ending, MonkeyParser::new(&mut id_map))(data).unwrap();
    assert_eq!(unparsed_input.trim(), "");
    let root_id = *id_map
        .get("root")
        .expect("couldn't find monkey with name 'root'");
    let my_id = *id_map
        .get("humn")
        .expect("couldn't find monkey with name 'humn'");

    monkeys.sort_by_key(|m| m.id);
    Input {
        monkeys,
        root_id,
        my_id,
    }
}

impl Input {
    fn eval(&mut self, id: Id) -> LargeVal {
        let m = &self.monkeys[id as usize];
        if let Some(res) = m.result {
            return res;
        }

        let res = match m.job {
            Job::Number(n) => n as LargeVal,
            Job::Computation(left_id, op, right_id) => {
                let left = self.eval(left_id);
                let right = self.eval(right_id);
                op.eval(left, right)
            }
        };
        self.monkeys[id as usize].result = Some(res);
        res
    }
}

#[aoc(day21, part1)]
pub fn part_1(input: &Input) -> LargeVal {
    let mut input = input.clone();
    input.eval(input.root_id)
}

enum ExpressionKind {
    Constant(LargeVal),
    Variable,
    Op(Box<Expression>, Operation, Box<Expression>),
}

struct Expression {
    kind: ExpressionKind,
    var_count: Option<u32>,
}

impl From<ExpressionKind> for Expression {
    fn from(kind: ExpressionKind) -> Self {
        Expression {
            kind,
            var_count: None,
        }
    }
}

impl Expression {
    fn new(input: &Input, id: Id) -> Expression {
        let kind = match input.monkeys[id as usize].job {
            Job::Number(_) if id == input.my_id => ExpressionKind::Variable,
            Job::Number(x) => ExpressionKind::Constant(x.into()),
            Job::Computation(left, op, right) => ExpressionKind::Op(
                Box::new(Expression::new(input, left)),
                op,
                Box::new(Expression::new(input, right)),
            ),
        };
        Expression {
            kind,
            var_count: None,
        }
    }

    fn var_count(&mut self) -> u32 {
        match self.var_count {
            Some(c) => c,
            None => {
                let count = match &mut self.kind {
                    ExpressionKind::Constant(_) => 0,
                    ExpressionKind::Variable => 1,
                    ExpressionKind::Op(left, _, right) => left.var_count() + right.var_count(),
                };
                self.var_count = Some(count);
                count
            }
        }
    }

    fn compute(&self) -> LargeVal {
        match &self.kind {
            ExpressionKind::Constant(x) => *x,
            ExpressionKind::Variable => panic!("Cannot compute expression containing a variable"),
            ExpressionKind::Op(x, op, y) => op.eval(x.compute(), y.compute()),
        }
    }
}

struct Equation {
    left: Expression,
    right: Expression,
}

impl Equation {
    fn new(input: &Input) -> Equation {
        let root = &input.monkeys[input.root_id as usize];
        if let Job::Computation(left, _, right) = root.job {
            return Equation {
                left: Expression::new(input, left),
                right: Expression::new(input, right),
            };
        }
        panic!("root monkey's value was not a binary operation")
    }

    fn solve(mut self) -> LargeVal {
        let l_var = self.left.var_count();
        let r_var = self.right.var_count();
        if l_var + r_var > 1 {
            panic!("This algorithm only handles the case where the variable is on one side of the equation.")
        } else if l_var + r_var == 0 {
            panic!("No variable found")
        }

        let (mut e_with_var, mut e_without_var) = if l_var > 0 {
            (self.left, self.right)
        } else {
            (self.right, self.left)
        };

        /*
            This loop iteratively unwraps the side of the expression that
            contains the variable until it contains nothing else, at which point we
            can evaluate the other side.
        */
        loop {
            match e_with_var.kind {
                ExpressionKind::Constant(_) => panic!("invalid state"),
                ExpressionKind::Variable => return e_without_var.compute(),
                ExpressionKind::Op(mut left, op, right) => {
                    if left.var_count() > 0 {
                        e_with_var = *left;
                        // (l +-*/ r = o) => (l = o -+/* r )
                        e_without_var =
                            ExpressionKind::Op(Box::new(e_without_var), op.inverse(), right).into();
                    } else {
                        e_with_var = *right;
                        let (new_l, new_op, new_r) = match op {
                            // (l +* r = o) => (o -/ l = r)
                            Operation::Add | Operation::Mul => {
                                (Box::new(e_without_var), op.inverse(), left)
                            }
                            // (l -/ r = o) => (l -/ o = r)
                            Operation::Sub | Operation::Div => (left, op, Box::new(e_without_var)),
                        };
                        e_without_var = ExpressionKind::Op(new_l, new_op, new_r).into();
                    }
                }
            }
        }
    }
}

#[aoc(day21, part2)]
pub fn part_2(input: &Input) -> LargeVal {
    let eq = Equation::new(input);
    eq.solve()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            root: pppw + sjmn
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            pppw: cczh / lfqf
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32
            "
        });
        assert_eq!(part_1(&input), 152);
        assert_eq!(part_2(&input), 301);
    }
}
