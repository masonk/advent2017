extern crate advent2017;
use advent2017::assembly;

#[test]
fn test_assembly_vm() {
    #[cfg(test)]
    mod example {
        use super::assembly::DuetState;
        use super::assembly::Inst::*;
        use super::assembly::Val::*;
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
