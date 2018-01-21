use assembly::Val;
use assembly::Inst;
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Instruction {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use assembly::Val;
    use assembly::Inst;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_22_23(&'input str),
        Termr_23_22add_22_23(&'input str),
        Termr_23_22jgz_22_23(&'input str),
        Termr_23_22mod_22_23(&'input str),
        Termr_23_22mul_22_23(&'input str),
        Termr_23_22rcv_22_23(&'input str),
        Termr_23_22set_22_23(&'input str),
        Termr_23_22snd_22_23(&'input str),
        NtAdd(Inst),
        NtAddr(Val),
        NtChar(char),
        NtInstruction(Inst),
        NtJgz(Inst),
        NtLit(Val),
        NtMod(Inst),
        NtMul(Inst),
        NtNum(i64),
        NtRcv(Inst),
        NtSet(Inst),
        NtSnd(Inst),
        NtVal(Val),
        Nt____Instruction(Inst),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 10, 11, 12, 13, 14, 15, 16,
        // State 1
        -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 2
        -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 3
        -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 4
        -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 5
        -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 6
        -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 7
        -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 8
        -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 9
        0, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 18
        -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 19
        -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 20
        -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 21
        -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 22
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 24
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 27
        24, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 29
        -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 30
        -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 31
        -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 32
        -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 33
        -17, -17, -17, -17, -17, -17, -17, -17, -17,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -6,
        -21,
        -10,
        -9,
        -8,
        -5,
        -7,
        -4,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -3,
        -19,
        -2,
        -20,
        -12,
        0,
        -15,
        0,
        0,
        -16,
        0,
        -18,
        -1,
        -11,
        -13,
        -14,
        -17,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 0, 3, 4, 0, 5, 6, 0, 7, 8, 9, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 23, 0,
        // State 11
        0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 27, 0,
        // State 14
        0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 29, 0,
        // State 16
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 30, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 31, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 32, 0,
        // State 25
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 33, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 19, 20, 0, 0, 21, 0, 0, 22, 0, 0, 0, 34, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"r#"-?[0-9]+"#"###,
            r###"r#"[a-z]"#"###,
            r###"r#"add"#"###,
            r###"r#"jgz"#"###,
            r###"r#"mod"#"###,
            r###"r#"mul"#"###,
            r###"r#"rcv"#"###,
            r###"r#"set"#"###,
            r###"r#"snd"#"###,
        ];
        __ACTION[(__state * 9)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Instruction<
        'input,
    >(
        input: &'input str,
    ) -> Result<Inst, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22add_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22jgz_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Termr_23_22mod_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Termr_23_22mul_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22rcv_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22set_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22snd_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Inst,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Add = r#"add"#, Char, Val => ActionFn(10);
                let __sym2 = __pop_NtVal(__symbols);
                let __sym1 = __pop_NtChar(__symbols);
                let __sym0 = __pop_Termr_23_22add_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            2 => {
                // Addr = Char => ActionFn(18);
                let __sym0 = __pop_NtChar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddr(__nt), __end));
                1
            }
            3 => {
                // Char = r#"[a-z]"# => ActionFn(20);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtChar(__nt), __end));
                2
            }
            4 => {
                // Instruction = Snd => ActionFn(1);
                let __sym0 = __pop_NtSnd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            5 => {
                // Instruction = Rcv => ActionFn(2);
                let __sym0 = __pop_NtRcv(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            6 => {
                // Instruction = Add => ActionFn(3);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            7 => {
                // Instruction = Set => ActionFn(4);
                let __sym0 = __pop_NtSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            8 => {
                // Instruction = Mul => ActionFn(5);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            9 => {
                // Instruction = Mod => ActionFn(6);
                let __sym0 = __pop_NtMod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            10 => {
                // Instruction = Jgz => ActionFn(7);
                let __sym0 = __pop_NtJgz(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInstruction(__nt), __end));
                3
            }
            11 => {
                // Jgz = r#"jgz"#, Val, Val => ActionFn(14);
                let __sym2 = __pop_NtVal(__symbols);
                let __sym1 = __pop_NtVal(__symbols);
                let __sym0 = __pop_Termr_23_22jgz_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtJgz(__nt), __end));
                4
            }
            12 => {
                // Lit = Num => ActionFn(17);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                5
            }
            13 => {
                // Mod = r#"mod"#, Char, Val => ActionFn(13);
                let __sym2 = __pop_NtVal(__symbols);
                let __sym1 = __pop_NtChar(__symbols);
                let __sym0 = __pop_Termr_23_22mod_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMod(__nt), __end));
                6
            }
            14 => {
                // Mul = r#"mul"#, Char, Val => ActionFn(12);
                let __sym2 = __pop_NtVal(__symbols);
                let __sym1 = __pop_NtChar(__symbols);
                let __sym0 = __pop_Termr_23_22mul_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                7
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                8
            }
            16 => {
                // Rcv = r#"rcv"#, Val => ActionFn(9);
                let __sym1 = __pop_NtVal(__symbols);
                let __sym0 = __pop_Termr_23_22rcv_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRcv(__nt), __end));
                9
            }
            17 => {
                // Set = r#"set"#, Char, Val => ActionFn(11);
                let __sym2 = __pop_NtVal(__symbols);
                let __sym1 = __pop_NtChar(__symbols);
                let __sym0 = __pop_Termr_23_22set_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSet(__nt), __end));
                10
            }
            18 => {
                // Snd = r#"snd"#, Val => ActionFn(8);
                let __sym1 = __pop_NtVal(__symbols);
                let __sym0 = __pop_Termr_23_22snd_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSnd(__nt), __end));
                11
            }
            19 => {
                // Val = Addr => ActionFn(15);
                let __sym0 = __pop_NtAddr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVal(__nt), __end));
                12
            }
            20 => {
                // Val = Lit => ActionFn(16);
                let __sym0 = __pop_NtLit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVal(__nt), __end));
                12
            }
            21 => {
                // __Instruction = Instruction => ActionFn(0);
                let __sym0 = __pop_NtInstruction(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dz_5d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dz_5d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22add_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22add_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22jgz_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22jgz_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22mod_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22mod_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22mul_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22mul_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22rcv_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22rcv_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22set_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22set_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22snd_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22snd_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAdd<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAdd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAddr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Val, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtChar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, char, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtChar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInstruction<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInstruction(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtJgz<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtJgz(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Val, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMod<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMod(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMul<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMul(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRcv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRcv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSnd<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSnd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Val, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Instruction<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inst, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Instruction(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Instruction::parse_Instruction;
mod __intern_token {
    #![allow(unused_imports)]
    use assembly::Val;
    use assembly::Inst;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\\-)?(?u:[0-9])+",
                "^(?u:[a-z])",
                "^(?u:add)",
                "^(?u:jgz)",
                "^(?u:mod)",
                "^(?u:mul)",
                "^(?u:rcv)",
                "^(?u:set)",
                "^(?u:snd)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\\-)?(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[a-z])").unwrap(),
                __regex::Regex::new("^(?u:add)").unwrap(),
                __regex::Regex::new("^(?u:jgz)").unwrap(),
                __regex::Regex::new("^(?u:mod)").unwrap(),
                __regex::Regex::new("^(?u:mul)").unwrap(),
                __regex::Regex::new("^(?u:rcv)").unwrap(),
                __regex::Regex::new("^(?u:set)").unwrap(),
                __regex::Regex::new("^(?u:snd)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 9 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Inst, usize),
) -> Inst
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Val, usize),
) -> Inst
{
    Inst::Snd(__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Val, usize),
) -> Inst
{
    Inst::Rcv(__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, char, usize),
    (_, __1, _): (usize, Val, usize),
) -> Inst
{
    Inst::Add(__0, __1)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, char, usize),
    (_, __1, _): (usize, Val, usize),
) -> Inst
{
    Inst::Set(__0, __1)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, char, usize),
    (_, __1, _): (usize, Val, usize),
) -> Inst
{
    Inst::Mul(__0, __1)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, char, usize),
    (_, __1, _): (usize, Val, usize),
) -> Inst
{
    Inst::Mod(__0, __1)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Val, usize),
    (_, __1, _): (usize, Val, usize),
) -> Inst
{
    Inst::Jgz(__0, __1)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Val, usize),
) -> Val
{
    (__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Val, usize),
) -> Val
{
    (__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Val
{
    Val::Lit(__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, char, usize),
) -> Val
{
    Val::Addr(__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> char
{
    s.chars().nth(0).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
