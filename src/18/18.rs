#[test]
fn test_assembly_vm() {
    #[cfg(test)]
    mod example {
        extern crate advent2017 as lib;
        use self::lib::assembly;
        use self::assembly::DuetState;
        use self::assembly::Inst::*;
        use self::assembly::Val::*;
        #[test]
        fn example_computes() {
            let insts = vec![
                Set('a', Lit(1)),
                Add('a', Lit(2)),
                Mul('a', Addr('a')),
                Mod('a', Lit(5)),
                Snd(Addr('a')),
                Set('a', Lit(0)),
                Rcv(Addr('a')),
                Jgz(Addr('a'), Lit(-1)),
                Set('a', Lit(1)),
                Jgz(Addr('a'), Lit(-2)),
            ];
            let mut duet = DuetState::new(insts);
            let actual = duet.play_until_recovery();
            assert_eq!(actual, 4);
        }
    }
}
#[cfg(test)]
mod test_parser {
    extern crate advent2017 as lib;
    use self::lib::assembly;
    use self::lib::assembly_instructions as parser;
    use self::assembly::DuetState;
    use self::assembly::Inst::*;
    use self::assembly::Val::*;

    #[test]
    fn parse_example() {
        let texts = vec![
            "set a 1", "add a 2", "mul a a", "mod a 5", "snd a", "set a 0", "rcv a", "jgz a -1",
            "set a 1", "jgz a -2",
        ];
        let parses = vec![
            Set('a', Lit(1)),
            Add('a', Lit(2)),
            Mul('a', Addr('a')),
            Mod('a', Lit(5)),
            Snd(Addr('a')),
            Set('a', Lit(0)),
            Rcv(Addr('a')),
            Jgz(Addr('a'), Lit(-1)),
            Set('a', Lit(1)),
            Jgz(Addr('a'), Lit(-2)),
        ];
        for (text, expected) in texts.iter().zip(parses) {
            let actual = parser::parse_Instruction(text);
            assert_eq!(actual.unwrap(), expected);
        }
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate advent2017 as lib;
use self::lib::assembly;
use self::lib::assembly_instructions as parser;
use self::assembly::DuetState;
use self::assembly::Inst;
use self::Inst::*;

fn part_one() -> i64 {
    let mut f = File::open("src/18/data").unwrap();
    let mut r = BufReader::new(f);
    let inst_texts = r.lines().map(|r| r.unwrap()).collect::<Vec<String>>();
    let insts = inst_texts
        .iter()
        .map(|ref l| parser::parse_Instruction(l))
        .map(|r| r.unwrap())
        .collect::<Vec<Inst>>();
    let mut duet = DuetState::new(insts);
    duet.play_until_recovery()
}

fn main() {
    println!("18-1: {}", part_one());
}
