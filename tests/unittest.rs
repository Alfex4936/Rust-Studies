fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("Negative".to_owned())
    }
}

struct Screen {
    x: i32,
    y: i32,
}

impl Screen {
    fn new(x: i32, y: i32) -> Screen {
        if x < 0 && y < 0 {
            panic!("X and Y must be positive.");
        }

        Screen { x, y }
    }

    fn update_screen(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn reset_screen(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    fn current_pos(&self) {
        println!("X: {}, Y: {}", self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_screen_panic() {
        Screen::new(-1, -3);
    }

    #[test]
    fn test_screen() {
        let mut s = Screen::new(1, 3);
        s.update_screen(3, 3);
        assert_eq!(s.x, 4);
        s.current_pos();
        s.reset_screen();
        assert_eq!(s.y, 0);
        s.current_pos();
    }
}
