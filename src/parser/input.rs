#[derive(Debug)]
crate struct Input<'a> {
    index: usize,
    lines: Vec<(usize, &'a str)>,
}

impl<'a> Input<'a> {
    crate fn new(lines: Vec<(usize, &'a str)>) -> Self {
        Input { index: 0, lines }
    }

    crate fn next(&mut self) -> Option<&(usize, &'a str)> {
        let result = self.lines.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result
    }

    crate fn peek(&mut self) -> Option<&(usize, &'a str)> {
        self.lines.get(self.index)
    }

    crate fn skip(&mut self) {
        self.index += 1;
    }
}
