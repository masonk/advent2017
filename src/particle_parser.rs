use particles::Vec3;
use particles::Particle3;
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Particle {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use particles::Vec3;
    use particles::Particle3;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Termr_23_22_2c_22_23(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_3d_3c_22_23(&'input str),
        Termr_23_22_3e_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_22_23(&'input str),
        NtChar(char),
        NtNum(i64),
        NtParticle(Particle3),
        NtVec3(Vec3),
        Nt____Particle(Particle3),
        Nt____Vec3(Vec3),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 5,
        // State 1
        0, 0, 6, 0, 0,
        // State 2
        -5, -5, -5, -5, -5,
        // State 3
        7, 0, 0, 0, 0,
        // State 4
        -1, -1, -1, -1, -1,
        // State 5
        0, 9, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 5,
        // State 7
        11, 0, 0, 0, 0,
        // State 8
        -2, -2, -2, -2, -2,
        // State 9
        12, 0, 0, 0, 0,
        // State 10
        0, 9, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 5,
        // State 12
        15, 0, 0, 0, 0,
        // State 13
        -3, -3, -3, -3, -3,
        // State 14
        0, 9, 0, 0, 0,
        // State 15
        0, 0, 0, 17, 0,
        // State 16
        -4, -4, -4, -4, -4,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        -5,
        0,
        -1,
        0,
        0,
        0,
        -2,
        0,
        0,
        0,
        0,
        -3,
        0,
        0,
        -4,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 3, 4, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0,
        // State 5
        0, 8, 0, 0, 0, 0,
        // State 6
        2, 0, 0, 10, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0,
        // State 10
        0, 13, 0, 0, 0, 0,
        // State 11
        2, 0, 0, 14, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0,
        // State 14
        0, 16, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"r#","#"###,
            r###"r#"-?[0-9]+"#"###,
            r###"r#"=<"#"###,
            r###"r#">"#"###,
            r###"r#"[a-z]"#"###,
        ];
        __ACTION[(__state * 5)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Particle<
        'input,
    >(
        input: &'input str,
    ) -> Result<Particle3, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                let __action = __ACTION[__state * 5 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_2c_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_3d_3c_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_3e_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_22_23((__tok0)),
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
    ) -> Option<Result<Particle3,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Char = r#"[a-z]"# => ActionFn(5);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtChar(__nt), __end));
                0
            }
            2 => {
                // Num = r#"-?[0-9]+"# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                1
            }
            3 => {
                // Particle = Vec3, r#","#, Vec3, r#","#, Vec3 => ActionFn(2);
                let __sym4 = __pop_NtVec3(__symbols);
                let __sym3 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym2 = __pop_NtVec3(__symbols);
                let __sym1 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym0 = __pop_NtVec3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtParticle(__nt), __end));
                2
            }
            4 => {
                // Vec3 = Char, r#"=<"#, Num, r#","#, Num, r#","#, Num, r#">"# => ActionFn(3);
                let __sym7 = __pop_Termr_23_22_3e_22_23(__symbols);
                let __sym6 = __pop_NtNum(__symbols);
                let __sym5 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym4 = __pop_NtNum(__symbols);
                let __sym3 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Termr_23_22_3d_3c_22_23(__symbols);
                let __sym0 = __pop_NtChar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtVec3(__nt), __end));
                3
            }
            5 => {
                // __Particle = Particle => ActionFn(0);
                let __sym0 = __pop_NtParticle(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            6 => {
                // __Vec3 = Vec3 => ActionFn(1);
                let __sym0 = __pop_NtVec3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Vec3(__nt), __end));
                5
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Termr_23_22_2c_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2c_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Termr_23_22_3d_3c_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_3d_3c_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_3e_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_3e_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtParticle<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Particle3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParticle(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVec3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVec3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Particle<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Particle3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Particle(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Vec3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Vec3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Particle::parse_Particle;

mod __parse__Vec3 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use particles::Vec3;
    use particles::Particle3;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Termr_23_22_2c_22_23(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_3d_3c_22_23(&'input str),
        Termr_23_22_3e_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_22_23(&'input str),
        NtChar(char),
        NtNum(i64),
        NtParticle(Particle3),
        NtVec3(Vec3),
        Nt____Particle(Particle3),
        Nt____Vec3(Vec3),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 4,
        // State 1
        0, 0, 5, 0, 0,
        // State 2
        -6, -6, -6, -6, -6,
        // State 3
        -1, -1, -1, -1, -1,
        // State 4
        0, 7, 0, 0, 0,
        // State 5
        8, 0, 0, 0, 0,
        // State 6
        -2, -2, -2, -2, -2,
        // State 7
        0, 7, 0, 0, 0,
        // State 8
        10, 0, 0, 0, 0,
        // State 9
        0, 7, 0, 0, 0,
        // State 10
        0, 0, 0, 12, 0,
        // State 11
        -4, -4, -4, -4, -4,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        -6,
        -1,
        0,
        0,
        -2,
        0,
        0,
        0,
        0,
        -4,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 0, 3, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0,
        // State 4
        0, 6, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0,
        // State 7
        0, 9, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0,
        // State 9
        0, 11, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"r#","#"###,
            r###"r#"-?[0-9]+"#"###,
            r###"r#"=<"#"###,
            r###"r#">"#"###,
            r###"r#"[a-z]"#"###,
        ];
        __ACTION[(__state * 5)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Vec3<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec3, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                let __action = __ACTION[__state * 5 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_2c_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_3d_3c_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_3e_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_22_23((__tok0)),
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
    ) -> Option<Result<Vec3,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Char = r#"[a-z]"# => ActionFn(5);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtChar(__nt), __end));
                0
            }
            2 => {
                // Num = r#"-?[0-9]+"# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                1
            }
            3 => {
                // Particle = Vec3, r#","#, Vec3, r#","#, Vec3 => ActionFn(2);
                let __sym4 = __pop_NtVec3(__symbols);
                let __sym3 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym2 = __pop_NtVec3(__symbols);
                let __sym1 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym0 = __pop_NtVec3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtParticle(__nt), __end));
                2
            }
            4 => {
                // Vec3 = Char, r#"=<"#, Num, r#","#, Num, r#","#, Num, r#">"# => ActionFn(3);
                let __sym7 = __pop_Termr_23_22_3e_22_23(__symbols);
                let __sym6 = __pop_NtNum(__symbols);
                let __sym5 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym4 = __pop_NtNum(__symbols);
                let __sym3 = __pop_Termr_23_22_2c_22_23(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Termr_23_22_3d_3c_22_23(__symbols);
                let __sym0 = __pop_NtChar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtVec3(__nt), __end));
                3
            }
            5 => {
                // __Particle = Particle => ActionFn(0);
                let __sym0 = __pop_NtParticle(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Particle(__nt), __end));
                4
            }
            6 => {
                // __Vec3 = Vec3 => ActionFn(1);
                let __sym0 = __pop_NtVec3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Termr_23_22_2c_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2c_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Termr_23_22_3d_3c_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_3d_3c_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_3e_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_3e_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtParticle<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Particle3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParticle(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVec3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVec3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Particle<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Particle3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Particle(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Vec3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Vec3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Vec3::parse_Vec3;
mod __intern_token {
    #![allow(unused_imports)]
    use particles::Vec3;
    use particles::Particle3;
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
                "^(?u:,)",
                "^(?u:\\-)?(?u:[0-9])+",
                "^(?u:=<)",
                "^(?u:>)",
                "^(?u:[a-z])",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\-)?(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:=<)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:[a-z])").unwrap(),
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
                    for __i in 0 .. 5 {
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
    (_, __0, _): (usize, Particle3, usize),
) -> Particle3
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec3, usize),
) -> Vec3
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec3, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec3, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Vec3, usize),
) -> Particle3
{
    Particle3::new(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, char, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, i64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, i64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, i64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec3
{
    Vec3::new(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action5<
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
