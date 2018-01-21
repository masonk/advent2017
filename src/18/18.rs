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
// pub mod assembly_instructions;

// #[test]
// fn test_assembly_parse() {
//     let lit = assembly_instructions::parse_Instruction("snd 1");
//     assert!(lit.is_ok());
//     assert_eq!(lit.unwrap(), Snd(Lit(1)));
//     let addr = assembly_instructions::parse_Instruction("snd a");
//     assert!(addr.is_ok());
//     assert!(addr, Snd(Addr('a')));
// }
