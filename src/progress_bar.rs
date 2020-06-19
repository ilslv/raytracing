pub(crate) struct ProgressBar {
    length: u32,
}

impl ProgressBar {
    pub fn new(length: u32) -> Self {
        ProgressBar { length }
    }

    pub fn update(&self, done: f32) {
        let progressed = (self.length as f32 * done) as u32;
        eprint!("\r|");
        (0..self.length)
            .for_each(|i| eprint!("{}", if i < progressed { "#" } else { "-" }));
        eprint!("|");
    }
}
