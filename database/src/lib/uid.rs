use std::collections::BTreeSet;

/// Incrementing uid that that uses the smallest available integer for the next uid or always
/// increments to the next uid.
pub struct IntCursor {
    pos: u128,
    free: BTreeSet<u128>,
}

impl IntCursor {
    pub fn new() -> IntCursor {
        IntCursor {
            pos: 0,
            free: BTreeSet::new(),
        }
    }

    pub fn next(&mut self) -> u128 {
        let free = &mut self.free;

        if free.len() == 0 {
            return self.increment();
        }

        free.pop_first().unwrap()
    }

    /// Returns true if the value existed or false if the value did not exist.
    pub fn drop(&mut self, pos: u128) -> bool {
        self.free.insert(pos);
        pos <= self.pos
    }

    fn increment(&mut self) -> u128 {
        let pos = self.pos;
        self.pos += 1;
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_increments_by_one() {
        let mut cursor = IntCursor::new();

        assert_eq!(cursor.next(), 0);
        assert_eq!(cursor.next(), 1);
    }

    #[test]
    fn it_uses_the_smallest_next_available_int() {
        let mut cursor = IntCursor::new();

        // Populate index to 5.
        for _ in 0..5 {
            cursor.next();
        }

        cursor.drop(3);
        cursor.drop(1);
        cursor.drop(4);

        assert_eq!(cursor.next(), 1);
    }
}
