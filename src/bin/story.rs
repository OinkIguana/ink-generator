pub mod story {
    #![allow(dead_code, unused_imports, unreachable_code, non_snake_case)]
    use inkgen::yield_all;
    pub const ID: inkgen::runtime::StoryID =
        inkgen::runtime::StoryID("c22d978f-e3d3-4b57-8ebc-414878dbe64e");
    pub fn story() -> inkgen::runtime::Story {
        let input = inkgen::runtime::Input::default();
        let state = inkgen::runtime::WrappedState::default();
        inkgen::runtime::Story::new(ID, input.clone(), state.clone(), move || loop {
            let choices = {
                let state = state.lock().unwrap();
                let mut choices = vec![];
                if false
                    || !state.visited(inkgen::runtime::StoryPoint::Unnamed(
                        "fd0c03f1-d908-426d-9d9a-b803a4dc5d1c",
                    )) {
                    choices.push(vec![inkgen::runtime::Part::Text("Sure")]);
                }
                if false
                    || !state.visited(inkgen::runtime::StoryPoint::Unnamed(
                        "a46901f8-9486-41d1-b164-d09ad9433889",
                    )) {
                    choices.push(vec![inkgen::runtime::Part::Text("Nope")]);
                }
                choices
            };
            yield inkgen::runtime::Paragraph::new(
                vec![inkgen::runtime::Part::Text(
                    "Hello. I think you\'re due for a story about now. Care to begin?",
                )],
                Some(choices),
            );
            let choice = *input.lock().unwrap();
            let mut i = 0;
            if false
                || !state
                    .lock()
                    .unwrap()
                    .visited(inkgen::runtime::StoryPoint::Unnamed(
                        "fd0c03f1-d908-426d-9d9a-b803a4dc5d1c",
                    )) {
                i += 1;
            }
            if i == choice {
                state
                    .lock()
                    .unwrap()
                    .visit(inkgen::runtime::StoryPoint::Unnamed(
                        "fd0c03f1-d908-426d-9d9a-b803a4dc5d1c",
                    ));
                yield inkgen::runtime::Paragraph::new(
                    vec![inkgen::runtime::Part::Text(
                        "Excellent. Let\'s get started then.",
                    )],
                    None,
                );
                let continuation = inkgen::runtime::Paragraph::new(vec![], None);
                let mut gen: Box<
                    dyn inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()>
                        + Sync
                        + Send,
                > = Box::new(knot_Story::entry(input, state));
                match unsafe { inkgen::runtime::Generator::resume(&mut gen) } {
                    inkgen::runtime::GeneratorState::Yielded(paragraph) => {
                        yield continuation.join(paragraph);
                        yield_all ! { gen }
                    }
                    inkgen::runtime::GeneratorState::Complete(()) => yield continuation,
                }
                break;
            }
            if false
                || !state
                    .lock()
                    .unwrap()
                    .visited(inkgen::runtime::StoryPoint::Unnamed(
                        "a46901f8-9486-41d1-b164-d09ad9433889",
                    )) {
                i += 1;
            }
            if i == choice {
                state
                    .lock()
                    .unwrap()
                    .visit(inkgen::runtime::StoryPoint::Unnamed(
                        "a46901f8-9486-41d1-b164-d09ad9433889",
                    ));
                yield inkgen::runtime::Paragraph::new(
                    vec![
                        inkgen::runtime::Part::Text(
                            "Oh, well um. I guess I\'ll be seeing you around then.",
                        ),
                        inkgen::runtime::Part::Tag("Lose"),
                    ],
                    None,
                );
                return;
                break;
            }
        })
    }
    mod knot_Story {
        use inkgen::yield_all;
        pub(super) fn entry(
            input: inkgen::runtime::Input,
            state: inkgen::runtime::WrappedState,
        ) -> impl inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()> + Sync + Send
        {
            move || {
                yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Text ( "Once upon a time, there were two children\u{2014}Alice and Bob, I\'ll call them." ) ] , None ) ;
                let continuation = inkgen::runtime::Paragraph::new(
                    vec![
                        inkgen::runtime::Part::Text(
                            "Now, you must know, Alice and Bob weren\'t exactly normal children...",
                        ),
                        inkgen::runtime::Part::Glue,
                    ],
                    None,
                );
                let mut gen: Box<
                    dyn inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()>
                        + Sync
                        + Send,
                > = Box::new(stitch_char_intro(input, state));
                match unsafe { inkgen::runtime::Generator::resume(&mut gen) } {
                    inkgen::runtime::GeneratorState::Yielded(paragraph) => {
                        yield continuation.join(paragraph);
                        yield_all ! { gen }
                    }
                    inkgen::runtime::GeneratorState::Complete(()) => yield continuation,
                }
            }
        }
        pub(super) fn stitch_char_intro(
            input: inkgen::runtime::Input,
            state: inkgen::runtime::WrappedState,
        ) -> impl inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()> + Sync + Send
        {
            move || loop {
                let choices = {
                    let state = state.lock().unwrap();
                    let mut choices = vec![];
                    if false || !state.visited(inkgen::runtime::StoryPoint::Named("alice")) {
                        choices.push(vec![
                            inkgen::runtime::Part::Text(""),
                            inkgen::runtime::Part::Text("Tell me about Alice"),
                        ]);
                    }
                    if false || !state.visited(inkgen::runtime::StoryPoint::Named("bob")) {
                        choices.push(vec![
                            inkgen::runtime::Part::Text(""),
                            inkgen::runtime::Part::Text("Tell me about Bob"),
                        ]);
                    }
                    if true
                        || !state.visited(inkgen::runtime::StoryPoint::Unnamed(
                            "b74f2a39-8086-4e8c-8425-866274185bac",
                        )) {
                        choices.push(vec![inkgen::runtime::Part::Text(
                            "That\'s all I need to know",
                        )]);
                    }
                    choices
                };
                yield inkgen::runtime::Paragraph::new(
                    vec![inkgen::runtime::Part::Text("Care to hear more?")],
                    Some(choices),
                );
                let choice = *input.lock().unwrap();
                let mut i = 0;
                if false
                    || !state
                        .lock()
                        .unwrap()
                        .visited(inkgen::runtime::StoryPoint::Named("alice"))
                {
                    i += 1;
                }
                if i == choice {
                    state
                        .lock()
                        .unwrap()
                        .visit(inkgen::runtime::StoryPoint::Named("alice"));
                    yield inkgen::runtime::Paragraph::new(
                        vec![inkgen::runtime::Part::Text("")],
                        None,
                    );
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Text ( "Well you see, Alice was well versed in the dark arts\u{2014}forbidden magics and the like. A master at" ) ] , None ) ;
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Glue , inkgen :: runtime :: Part :: Text ( "potions like none other. Needless to say, a dangerous character. She didn\'t keep many friends," ) ] , None ) ;
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Glue , inkgen :: runtime :: Part :: Text ( "or rather, many friends didn\'t keep her. Those who didn\'t recognize her for who she truly was," ) ] , None ) ;
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Glue , inkgen :: runtime :: Part :: Text ( "well... I can\'t say it turned out well for them. Other than Bob that is." ) ] , None ) ;
                    let continuation = inkgen::runtime::Paragraph::new(vec![], None);
                    let mut gen: Box<
                        dyn inkgen::runtime::Generator<
                                Yield = inkgen::runtime::Paragraph,
                                Return = (),
                            > + Sync
                            + Send,
                    > = Box::new(stitch_char_intro(input, state));
                    match unsafe { inkgen::runtime::Generator::resume(&mut gen) } {
                        inkgen::runtime::GeneratorState::Yielded(paragraph) => {
                            yield continuation.join(paragraph);
                            yield_all ! { gen }
                        }
                        inkgen::runtime::GeneratorState::Complete(()) => yield continuation,
                    }
                    break;
                }
                if false
                    || !state
                        .lock()
                        .unwrap()
                        .visited(inkgen::runtime::StoryPoint::Named("bob"))
                {
                    i += 1;
                }
                if i == choice {
                    state
                        .lock()
                        .unwrap()
                        .visit(inkgen::runtime::StoryPoint::Named("bob"));
                    yield inkgen::runtime::Paragraph::new(
                        vec![inkgen::runtime::Part::Text("")],
                        None,
                    );
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Text ( "Bob was a man of intelligence above all others. I have never seen anyone else who had a grasp of" ) ] , None ) ;
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Glue , inkgen :: runtime :: Part :: Text ( "such a variety of subjects as Bob. The only thing he seemed unable to master was magic. The" ) ] , None ) ;
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Glue , inkgen :: runtime :: Part :: Text ( "fundamentally non-sensical nature of magic didn\'t sit well in his entirely too rational mind." ) ] , None ) ;
                    yield inkgen :: runtime :: Paragraph :: new ( vec ! [ inkgen :: runtime :: Part :: Glue , inkgen :: runtime :: Part :: Text ( "Maybe Alice\'s skills were what inspired Bob to stick around with her all this time." ) ] , None ) ;
                    let continuation = inkgen::runtime::Paragraph::new(vec![], None);
                    let mut gen: Box<
                        dyn inkgen::runtime::Generator<
                                Yield = inkgen::runtime::Paragraph,
                                Return = (),
                            > + Sync
                            + Send,
                    > = Box::new(stitch_char_intro(input, state));
                    match unsafe { inkgen::runtime::Generator::resume(&mut gen) } {
                        inkgen::runtime::GeneratorState::Yielded(paragraph) => {
                            yield continuation.join(paragraph);
                            yield_all ! { gen }
                        }
                        inkgen::runtime::GeneratorState::Complete(()) => yield continuation,
                    }
                    break;
                }
                if true
                    || !state
                        .lock()
                        .unwrap()
                        .visited(inkgen::runtime::StoryPoint::Unnamed(
                            "b74f2a39-8086-4e8c-8425-866274185bac",
                        )) {
                    i += 1;
                }
                if i == choice {
                    state
                        .lock()
                        .unwrap()
                        .visit(inkgen::runtime::StoryPoint::Unnamed(
                            "b74f2a39-8086-4e8c-8425-866274185bac",
                        ));
                    yield inkgen::runtime::Paragraph::new(
                        vec![inkgen::runtime::Part::Text(
                            "Well, lucky for you, that\'s all I have to tell.",
                        )],
                        None,
                    );
                    yield inkgen::runtime::Paragraph::new(
                        vec![
                            inkgen::runtime::Part::Text("See you around, friend."),
                            inkgen::runtime::Part::Tag("Win"),
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
