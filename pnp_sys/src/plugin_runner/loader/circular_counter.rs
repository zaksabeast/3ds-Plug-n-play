#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CircularCounter {
    value: usize,
    min: usize,
    max: usize,
}

impl CircularCounter {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            value: min,
            min,
            max,
        }
    }
    pub fn value(&self) -> usize {
        self.value
    }

    pub fn increment(&mut self) -> usize {
        if self.value == self.max {
            self.value = self.min;
        } else {
            self.value += 1;
        }

        self.value
    }

    pub fn decrement(&mut self) -> usize {
        if self.value == self.min {
            self.value = self.max;
        } else {
            self.value -= 1;
        }

        self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod value {
        use super::*;

        #[test]
        fn should_return_value() {
            let counter = CircularCounter {
                value: 1,
                min: 0,
                max: 10,
            };
            assert_eq!(counter.value(), 1);
        }
    }

    mod increment {
        use super::*;

        #[test]
        fn should_increment() {
            let mut counter = CircularCounter {
                value: 1,
                min: 0,
                max: 10,
            };
            let result = counter.increment();
            assert_eq!(result, 2);
            assert_eq!(
                counter,
                CircularCounter {
                    value: 2,
                    min: 0,
                    max: 10
                }
            );
        }

        #[test]
        fn should_increment_to_min_on_overflow() {
            let mut counter = CircularCounter {
                value: 3,
                min: 1,
                max: 3,
            };
            let result = counter.increment();
            assert_eq!(result, 1);
            assert_eq!(
                counter,
                CircularCounter {
                    value: 1,
                    min: 1,
                    max: 3
                }
            );
        }
    }

    mod decrement {
        use super::*;

        #[test]
        fn should_decrement() {
            let mut counter = CircularCounter {
                value: 2,
                min: 1,
                max: 10,
            };
            let result = counter.decrement();
            assert_eq!(result, 1);
            assert_eq!(
                counter,
                CircularCounter {
                    value: 1,
                    min: 1,
                    max: 10
                }
            );
        }

        #[test]
        fn should_decrement_to_max_on_overflow() {
            let mut counter = CircularCounter {
                value: 1,
                min: 1,
                max: 3,
            };
            let result = counter.decrement();
            assert_eq!(result, 3);
            assert_eq!(
                counter,
                CircularCounter {
                    value: 3,
                    min: 1,
                    max: 3,
                }
            );
        }
    }
}
