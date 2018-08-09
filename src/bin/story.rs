pub mod story {
    #![allow(dead_code, unused_imports, unreachable_code, non_snake_case)]
    use inkgen::runtime as inkgen;
    use inkgen::yield_all;
    pub fn story() -> inkgen::Story {
        let input = inkgen::Input::default();
        let state = inkgen::WrappedState::default();
        inkgen::Story::new(input.clone(), state.clone(), move || loop {
            let choices = {
                let state = state.lock().unwrap();
                let mut choices = vec![];
                if false || !state.visited(inkgen::StoryPoint::Unnamed(
                    "7981688a-68c6-4dcc-b561-7099c08279a4",
                )) {
                    choices.push(vec![inkgen::Part::Text(String::from("Sure"))]);
                }
                if false || !state.visited(inkgen::StoryPoint::Unnamed(
                    "17ed9ace-e973-44a8-869e-f14b71f3a38a",
                )) {
                    choices.push(vec![inkgen::Part::Text(String::from("Nope"))]);
                }
                choices
            };
            yield inkgen::Paragraph::new(
                vec![inkgen::Part::Text(String::from(
                    "Hello. I think you\'re due for a story about now. Care to begin?",
                ))],
                Some(choices),
            );
            let choice = *input.lock().unwrap();
            let mut i = 0;
            if false || !state.lock().unwrap().visited(inkgen::StoryPoint::Unnamed(
                "7981688a-68c6-4dcc-b561-7099c08279a4",
            )) {
                i += 1;
            }
            if i == choice {
                state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                    "7981688a-68c6-4dcc-b561-7099c08279a4",
                ));
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from(
                        "Excellent. Let\'s get started then.",
                    ))],
                    None,
                );
                let continuation = inkgen::Paragraph::new(vec![], None);
                let mut gen: Box<
                    dyn inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> + Sync + Send,
                > = Box::new(knot_Story::entry(input, state));
                match unsafe { inkgen::Generator::resume(&mut gen) } {
                    inkgen::GeneratorState::Yielded(paragraph) => {
                        yield continuation.join(paragraph);
                        yield_all ! { gen }
                    }
                    inkgen::GeneratorState::Complete(()) => yield continuation,
                }
                break;
            }
            if false || !state.lock().unwrap().visited(inkgen::StoryPoint::Unnamed(
                "17ed9ace-e973-44a8-869e-f14b71f3a38a",
            )) {
                i += 1;
            }
            if i == choice {
                state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                    "17ed9ace-e973-44a8-869e-f14b71f3a38a",
                ));
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from(
                        "Oh, well um. I guess I\'ll be seeing you around then.",
                    ))],
                    None,
                );
                return;
                break;
            }
        })
    }
    mod knot_Story {
        use inkgen::runtime as inkgen;
        use inkgen::yield_all;
        pub(super) fn entry(
            input: inkgen::Input,
            state: inkgen::WrappedState,
        ) -> impl inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> + Sync + Send {
            move || {
                yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Once upon a time, there were two children\u{2014}Alice and Bob, I\'ll call them." ) ) ] , None ) ;
                let continuation = inkgen::Paragraph::new(
                    vec![
                        inkgen::Part::Text(String::from(
                            "Now, you must know, Alice and Bob weren\'t exactly normal children...",
                        )),
                        inkgen::Part::Glue,
                    ],
                    None,
                );
                let mut gen: Box<
                    dyn inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> + Sync + Send,
                > = Box::new(stitch_char_intro(input, state));
                match unsafe { inkgen::Generator::resume(&mut gen) } {
                    inkgen::GeneratorState::Yielded(paragraph) => {
                        yield continuation.join(paragraph);
                        yield_all ! { gen }
                    }
                    inkgen::GeneratorState::Complete(()) => yield continuation,
                }
            }
        }
        pub(super) fn stitch_char_intro(
            input: inkgen::Input,
            state: inkgen::WrappedState,
        ) -> impl inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> + Sync + Send {
            move || loop {
                let choices = {
                    let state = state.lock().unwrap();
                    let mut choices = vec![];
                    if false || !state.visited(inkgen::StoryPoint::Unnamed(
                        "ad706d42-21d9-4afc-8abc-d15bee66d837",
                    )) {
                        choices.push(vec![inkgen::Part::Text(String::from(
                            "Tell me about Alice",
                        ))]);
                    }
                    if false || !state.visited(inkgen::StoryPoint::Unnamed(
                        "a1a3da66-4b69-4856-a7cb-8e9c352e47d8",
                    )) {
                        choices.push(vec![inkgen::Part::Text(String::from("Tell me about Bob"))]);
                    }
                    if true || !state.visited(inkgen::StoryPoint::Unnamed(
                        "aaeec53d-6a3d-4b6f-aec6-f0899bf3e88f",
                    )) {
                        choices.push(vec![inkgen::Part::Text(String::from(
                            "That\'s all I need to know",
                        ))]);
                    }
                    choices
                };
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from("Care to hear more?"))],
                    Some(choices),
                );
                let choice = *input.lock().unwrap();
                let mut i = 0;
                if false || !state.lock().unwrap().visited(inkgen::StoryPoint::Unnamed(
                    "ad706d42-21d9-4afc-8abc-d15bee66d837",
                )) {
                    i += 1;
                }
                if i == choice {
                    state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                        "ad706d42-21d9-4afc-8abc-d15bee66d837",
                    ));
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Well you see, Alice was well versed in the dark arts\u{2014}forbidden magics and the like. A master at" ) ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( String :: from ( "potions like none other. Needless to say, a dangerous character. She didn\'t keep many friends," ) ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( String :: from ( "or rather, many friends didn\'t keep her. Those who didn\'t recognize her for who she truly was," ) ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( String :: from ( "well... I can\'t say it turned out well for them. Other than Bob that is." ) ) ] , None ) ;
                    let continuation = inkgen::Paragraph::new(vec![], None);
                    let mut gen: Box<
                        dyn inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> + Sync + Send,
                    > = Box::new(stitch_char_intro(input, state));
                    match unsafe { inkgen::Generator::resume(&mut gen) } {
                        inkgen::GeneratorState::Yielded(paragraph) => {
                            yield continuation.join(paragraph);
                            yield_all ! { gen }
                        }
                        inkgen::GeneratorState::Complete(()) => yield continuation,
                    }
                    break;
                }
                if false || !state.lock().unwrap().visited(inkgen::StoryPoint::Unnamed(
                    "a1a3da66-4b69-4856-a7cb-8e9c352e47d8",
                )) {
                    i += 1;
                }
                if i == choice {
                    state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                        "a1a3da66-4b69-4856-a7cb-8e9c352e47d8",
                    ));
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Bob was a man of intelligence above all others. I have never seen anyone else who had a grasp of" ) ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( String :: from ( "such a variety of subjects as Bob. The only thing he seemed unable to master was magic. The" ) ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( String :: from ( "fundamentally non-sensical nature of magic didn\'t sit well in his entirely too rational mind." ) ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( String :: from ( "Maybe Alice\'s skills were what inspired Bob to stick around with her all this time." ) ) ] , None ) ;
                    let continuation = inkgen::Paragraph::new(vec![], None);
                    let mut gen: Box<
                        dyn inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> + Sync + Send,
                    > = Box::new(stitch_char_intro(input, state));
                    match unsafe { inkgen::Generator::resume(&mut gen) } {
                        inkgen::GeneratorState::Yielded(paragraph) => {
                            yield continuation.join(paragraph);
                            yield_all ! { gen }
                        }
                        inkgen::GeneratorState::Complete(()) => yield continuation,
                    }
                    break;
                }
                if true || !state.lock().unwrap().visited(inkgen::StoryPoint::Unnamed(
                    "aaeec53d-6a3d-4b6f-aec6-f0899bf3e88f",
                )) {
                    i += 1;
                }
                if i == choice {
                    state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                        "aaeec53d-6a3d-4b6f-aec6-f0899bf3e88f",
                    ));
                    yield inkgen::Paragraph::new(
                        vec![inkgen::Part::Text(String::from(
                            "Well, lucky for you, that\'s all I have to tell.",
                        ))],
                        None,
                    );
                    yield inkgen::Paragraph::new(
                        vec![inkgen::Part::Text(String::from("See you around, friend."))],
                        None,
                    );
                    return;
                    break;
                }
            }
        }
    }
}
