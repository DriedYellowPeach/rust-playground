
#[derive(Debug, Default)]
pub struct App {
    pub should_exit: bool,
    pub counter: i64,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {

    }

    pub fn quit(&mut self) {
        self.should_exit = true;
    }

    pub fn incremenet(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement(&mut self) {
        if let Some(res) = self.counter.checked_add(-1) {
            self.counter = res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_increment_counter() {
        let mut app = App::new();
        app.incremenet();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::new();
        app.decrement();
        assert_eq!(app.counter, -1);
    }

}
