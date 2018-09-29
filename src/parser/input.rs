#[derive(Debug)]
pub(crate) struct Input<'a> {
    index: usize,
    lines: Vec<(usize, &'a str)>,
}

impl<'a> Input<'a> {
    pub(crate) fn new(lines: Vec<(usize, &'a str)>) -> Self {
        Input { index: 0, lines }
    }

    pub(crate) fn next(&mut self) -> Option<&(usize, &'a str)> {
        let result = self.lines.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result
    }

    pub(crate) fn peek(&mut self) -> Option<&(usize, &'a str)> {
        self.lines.get(self.index)
    }

    pub(crate) fn skip(&mut self) {
        self.index += 1;
    }
}
