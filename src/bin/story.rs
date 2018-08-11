pub mod story {
    #![allow(dead_code, unused_imports, unreachable_code, non_snake_case)]
    use inkgen::runtime as inkgen;
    use inkgen::yield_all;
    pub const ID: inkgen::StoryID = inkgen::StoryID("f8ebd703-469b-4e1f-bb9b-bb54669d680f");
    pub fn story() -> inkgen::Story {
        let input = inkgen::Input::default();
        let state = inkgen::WrappedState::default();
        inkgen::Story::new(ID, input.clone(), state.clone(), move || loop {
            let choices = {
                let state = state.lock().unwrap();
                let mut choices = vec![];
                if false || !state.visited(inkgen::StoryPoint::Unnamed(
                    "cdfe4b24-582c-435c-b8e9-5bb2440c513e",
                )) {
                    choices.push(vec![inkgen::Part::Text("Sure")]);
                }
                if false || !state.visited(inkgen::StoryPoint::Unnamed(
                    "865c21b6-cb1d-4ad7-92aa-0d860b820551",
                )) {
                    choices.push(vec![inkgen::Part::Text("Nope")]);
                }
                choices
            };
            yield inkgen::Paragraph::new(
                vec![inkgen::Part::Text(
                    "Hello. I think you\'re due for a story about now. Care to begin?",
                )],
                Some(choices),
            );
            let choice = *input.lock().unwrap();
            let mut i = 0;
            if false || !state.lock().unwrap().visited(inkgen::StoryPoint::Unnamed(
                "cdfe4b24-582c-435c-b8e9-5bb2440c513e",
            )) {
                i += 1;
            }
            if i == choice {
                state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                    "cdfe4b24-582c-435c-b8e9-5bb2440c513e",
                ));
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text("Excellent. Let\'s get started then.")],
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
                "865c21b6-cb1d-4ad7-92aa-0d860b820551",
            )) {
                i += 1;
            }
            if i == choice {
                state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                    "865c21b6-cb1d-4ad7-92aa-0d860b820551",
                ));
                yield inkgen::Paragraph::new(
                    vec![
                        inkgen::Part::Text("Oh, well um. I guess I\'ll be seeing you around then."),
                        inkgen::Part::Tag("Lose"),
                    ],
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
                yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( "Once upon a time, there were two children\u{2014}Alice and Bob, I\'ll call them." ) ] , None ) ;
                let continuation = inkgen::Paragraph::new(
                    vec![
                        inkgen::Part::Text(
                            "Now, you must know, Alice and Bob weren\'t exactly normal children...",
                        ),
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
                    if false || !state.visited(inkgen::StoryPoint::Named("alice")) {
                        choices.push(vec![
                            inkgen::Part::Text(""),
                            inkgen::Part::Text("Tell me about Alice"),
                        ]);
                    }
                    if false || !state.visited(inkgen::StoryPoint::Named("bob")) {
                        choices.push(vec![
                            inkgen::Part::Text(""),
                            inkgen::Part::Text("Tell me about Bob"),
                        ]);
                    }
                    if true || !state.visited(inkgen::StoryPoint::Unnamed(
                        "b967289a-8f54-456c-9a4f-e003e401f998",
                    )) {
                        choices.push(vec![inkgen::Part::Text("That\'s all I need to know")]);
                    }
                    choices
                };
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text("Care to hear more?")],
                    Some(choices),
                );
                let choice = *input.lock().unwrap();
                let mut i = 0;
                if false || !state
                    .lock()
                    .unwrap()
                    .visited(inkgen::StoryPoint::Named("alice"))
                {
                    i += 1;
                }
                if i == choice {
                    state
                        .lock()
                        .unwrap()
                        .visit(inkgen::StoryPoint::Named("alice"));
                    yield inkgen::Paragraph::new(vec![inkgen::Part::Text("")], None);
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( "Well you see, Alice was well versed in the dark arts\u{2014}forbidden magics and the like. A master at" ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( "potions like none other. Needless to say, a dangerous character. She didn\'t keep many friends," ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( "or rather, many friends didn\'t keep her. Those who didn\'t recognize her for who she truly was," ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( "well... I can\'t say it turned out well for them. Other than Bob that is." ) ] , None ) ;
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
                if false || !state
                    .lock()
                    .unwrap()
                    .visited(inkgen::StoryPoint::Named("bob"))
                {
                    i += 1;
                }
                if i == choice {
                    state
                        .lock()
                        .unwrap()
                        .visit(inkgen::StoryPoint::Named("bob"));
                    yield inkgen::Paragraph::new(vec![inkgen::Part::Text("")], None);
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( "Bob was a man of intelligence above all others. I have never seen anyone else who had a grasp of" ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( "such a variety of subjects as Bob. The only thing he seemed unable to master was magic. The" ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( "fundamentally non-sensical nature of magic didn\'t sit well in his entirely too rational mind." ) ] , None ) ;
                    yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Glue , inkgen :: Part :: Text ( "Maybe Alice\'s skills were what inspired Bob to stick around with her all this time." ) ] , None ) ;
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
                    "b967289a-8f54-456c-9a4f-e003e401f998",
                )) {
                    i += 1;
                }
                if i == choice {
                    state.lock().unwrap().visit(inkgen::StoryPoint::Unnamed(
                        "b967289a-8f54-456c-9a4f-e003e401f998",
                    ));
                    yield inkgen::Paragraph::new(
                        vec![inkgen::Part::Text(
                            "Well, lucky for you, that\'s all I have to tell.",
                        )],
                        None,
                    );
                    yield inkgen::Paragraph::new(
                        vec![
                            inkgen::Part::Text("See you around, friend."),
                            inkgen::Part::Tag("Win"),
                        ],
                        None,
                    );
                    return;
                    break;
                }
            }
        }
    }
}
