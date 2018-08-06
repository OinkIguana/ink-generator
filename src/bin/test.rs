pub mod story {
    #![allow(dead_code)]
    use inkgen::runtime as inkgen;
    use inkgen::yield_all;
    pub fn story() -> inkgen::Story {
        let input: inkgen::Rc<inkgen::Cell<usize>> = inkgen::Rc::default();
        inkgen::Story::new(input.clone(), move || {
            loop {
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from(
                        "\"What\'s that?\" my master asked.",
                    ))],
                    Some(vec![
                        vec![
                            inkgen::Part::Text(String::from("\"I am somewhat tired")),
                            inkgen::Part::Text(String::from(".\"")),
                        ],
                        vec![inkgen::Part::Text(String::from("\"Nothing, Monsieur!\""))],
                        vec![
                            inkgen::Part::Text(String::from("\"I said, this journey is appalling")),
                            inkgen::Part::Text(String::from(".\"")),
                        ],
                    ]),
                );
                let choice = input.get();
                match choice {
                    0usize => {
                        yield inkgen::Paragraph::new(
                            vec![
                                inkgen::Part::Text(String::from("\"I am somewhat tired")),
                                inkgen::Part::Text(String::from(",\" I repeated.")),
                            ],
                            None,
                        );
                        yield inkgen::Paragraph::new(
                            vec![inkgen::Part::Text(String::from(
                                "\"Really,\" he responded. \"How deleterious.\"",
                            ))],
                            None,
                        );
                        break;
                    }
                    1usize => {
                        yield inkgen::Paragraph::new(
                            vec![
                                inkgen::Part::Text(String::from("\"Nothing, Monsieur!\"")),
                                inkgen::Part::Text(String::from("I replied.")),
                            ],
                            None,
                        );
                        yield inkgen::Paragraph::new(
                            vec![inkgen::Part::Text(String::from("\"Very good, then.\""))],
                            None,
                        );
                        break;
                    }
                    2usize => {
                        yield inkgen::Paragraph::new(
                            vec![
                                inkgen::Part::Text(String::from(
                                    "\"I said, this journey is appalling",
                                )),
                                inkgen::Part::Text(String::from("and I want no more of it.\"")),
                            ],
                            None,
                        );
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "\"Ah,\" he replied, not unkindly. \"I see you are feeling frustrated. Tomorrow, things will improve.\"" ) ) ] , None ) ;
                        break;
                    }
                    _ => continue,
                }
            }
            let continuation = inkgen::Paragraph::new(vec![], None);
            let mut gen = knot_tomorrow::stitch_morning(input.clone());
            match unsafe { inkgen::Generator::resume(&mut gen) } {
                inkgen::GeneratorState::Yielded(paragraph) => {
                    yield continuation.join(paragraph);
                    yield_all ! { gen }
                }
                inkgen::GeneratorState::Complete(()) => yield continuation,
            }
        })
    }
    mod knot_tomorrow {
        use inkgen::runtime as inkgen;
        pub(super) fn entry(
            input: inkgen::Rc<inkgen::Cell<usize>>,
        ) -> impl inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> {
            stitch_morning(input.clone())
        }
        pub(super) fn stitch_morning(
            input: inkgen::Rc<inkgen::Cell<usize>>,
        ) -> impl inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> {
            move || {
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from(
                        "I heard a mumbled \"Good morning...\" as my master rolled out of bed.",
                    ))],
                    None,
                );
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from(
                        "\"Are you alright, Monsieur?\" I asked.",
                    ))],
                    None,
                );
                loop {
                    yield inkgen::Paragraph::new(
                        vec![inkgen::Part::Text(String::from(
                            "\"Quite right,\" he seemed to try to say.",
                        ))],
                        Some(vec![
                            vec![inkgen::Part::Text(String::from(
                                "\"Are you quite sure of that?\"",
                            ))],
                            vec![
                                inkgen::Part::Text(String::from("\"You don\'t look quite right")),
                                inkgen::Part::Text(String::from(".\"")),
                            ],
                            vec![
                                inkgen::Part::Text(String::from("\"Very well")),
                                inkgen::Part::Text(String::from(".\"")),
                            ],
                        ]),
                    );
                    let choice = input.get();
                    match choice {
                        0usize => {
                            yield inkgen::Paragraph::new(
                                vec![inkgen::Part::Text(String::from(
                                    "\"Are you quite sure of that?\"",
                                ))],
                                None,
                            );
                            yield inkgen::Paragraph::new(
                                vec![inkgen::Part::Text(String::from(
                                    "\"Yes, yes, don\'t you worry about me.\"",
                                ))],
                                None,
                            );
                            break;
                        }
                        1usize => {
                            yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "\"You don\'t look quite right" ) ) , inkgen :: Part :: Text ( String :: from ( ", Monsieur,\" I felt the need to say, \"You look rather tired.\"" ) ) ] , None ) ;
                            yield inkgen::Paragraph::new(
                                vec![inkgen::Part::Text(String::from(
                                    "\"Well, yes... but that is of no issue.\"",
                                ))],
                                None,
                            );
                            break;
                        }
                        2usize => {
                            yield inkgen::Paragraph::new(
                                vec![
                                    inkgen::Part::Text(String::from("\"Very well")),
                                    inkgen::Part::Text(String::from(
                                        ", call if you need me,\" I say, as I leave the room.",
                                    )),
                                ],
                                None,
                            );
                            yield inkgen::Paragraph::new(
                                vec![inkgen::Part::Text(String::from("\"Yes, very well.\""))],
                                None,
                            );
                            break;
                        }
                        _ => continue,
                    }
                }
                return;
            }
        }
    }
}
