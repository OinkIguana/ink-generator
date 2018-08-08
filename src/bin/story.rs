pub mod story {
    #![allow(dead_code, unused_imports)]
    use inkgen::runtime as inkgen;
    use inkgen::yield_all;
    pub fn story() -> inkgen::Story {
        let input: inkgen::Arc<inkgen::Mutex<usize>> = inkgen::Arc::default();
        inkgen::Story::new(input.clone(), move || {
            yield inkgen::Paragraph::new(
                vec![inkgen::Part::Text(String::from(
                    "You: <thought:Something feels off...>",
                ))],
                None,
            );
            yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "You: Who\'s in there? You\'d best come out quick. I don\'t take kindly to being snuck up on." ) ) ] , None ) ;
            loop {
                yield inkgen::Paragraph::new(
                    vec![inkgen::Part::Text(String::from(
                        "(Voice): Heh. You\'re a sharp one...",
                    ))],
                    Some(vec![
                        vec![inkgen::Part::Text(String::from("..."))],
                        vec![
                            inkgen::Part::Text(String::from("You: Watch it")),
                            inkgen::Part::Text(String::from(".")),
                        ],
                    ]),
                );
                let choice = *input.lock().unwrap();
                match choice {
                    0usize => {
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Mystery Man: ... but I knew that already. Your reputation precedes you, kid." ) ) ] , None ) ;
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Mystery Man: I\'ve been looking for talent like yours. Heard it could be found around here." ) ) ] , None ) ;
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Mystery Man: I realize you\'ve got some cops to be getting away from right now, so why don\'t you" ) ) , inkgen :: Part :: Glue ] , None ) ;
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "just take this and give me a call everything\'s all settled down again, hm?" ) ) ] , None ) ;
                        yield inkgen::Paragraph::new(
                            vec![inkgen::Part::Text(String::from("You: ... What is it?"))],
                            None,
                        );
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Mystery Man: A proposition... of a sort. I think you\'ll be interested. Now if you\'ll excuse me," ) ) , inkgen :: Part :: Glue ] , None ) ;
                        yield inkgen::Paragraph::new(
                            vec![inkgen::Part::Text(String::from(
                                "I\'ve some else place to be right now.",
                            ))],
                            None,
                        );
                        break;
                    }
                    1usize => {
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "You: Watch it" ) ) , inkgen :: Part :: Text ( String :: from ( ", mate. I think you might have found yourself in the wrong part of town." ) ) ] , None ) ;
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Mystery Man: Whoa, no need to get all worked up. I have come on business\u{2013}" ) ) ] , None ) ;
                        yield inkgen::Paragraph::new(
                            vec![inkgen::Part::Text(String::from(
                                "You: Well hurry it up then. I haven\'t got all day.",
                            ))],
                            None,
                        );
                        yield inkgen :: Paragraph :: new ( vec ! [ inkgen :: Part :: Text ( String :: from ( "Mystery Man: Well, why don\'t I just leave you with this then, and I\'ll be on my way. I trust" ) ) , inkgen :: Part :: Glue ] , None ) ;
                        yield inkgen::Paragraph::new(
                            vec![inkgen::Part::Text(String::from(
                                "I\'ll be hearing from you again soon.",
                            ))],
                            None,
                        );
                        break;
                    }
                    _ => continue,
                }
            }
        })
    }
}
