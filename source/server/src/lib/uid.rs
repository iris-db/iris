//! Unique value generation.
use std::collections::BTreeSet;

/// Unique value generation strategy.
pub trait UidGenerator {
    type Value;

    /// Generates a new unique value.
    fn generate_uid(&mut self) -> Self::Value;
}

/// Free an inaccessible uid.
pub trait UidDropper {
    type Value;

    /// Drops a uid, returning true if it existed in the uid sequence and false if it did not.
    fn drop_uid(&mut self, uid: Self::Value) -> bool;
}

/// Incrementing uid that that uses the smallest available integer for the next uid or always
/// increments to the next uid.
pub struct IntUid {
    /// Uid position.
    pos: u64,
    /// Free uid queue.
    free: BTreeSet<u64>,
}

impl UidGenerator for IntUid {
    type Value = u64;

    fn generate_uid(&mut self) -> Self::Value {
        self.next()
    }
}

impl UidDropper for IntUid {
    type Value = u64;

    fn drop_uid(&mut self, uid: Self::Value) -> bool {
        self.drop(uid)
    }
}

impl IntUid {
    pub fn new() -> IntUid {
        IntUid {
            pos: 0,
            free: BTreeSet::new(),
        }
    }

    pub fn next(&mut self) -> u64 {
        let free = &mut self.free;

        if free.len() == 0 {
            return self.increment();
        }

        free.pop_first().unwrap()
    }

    /// Returns true if the value existed or false if the value did not exist.
    pub fn drop(&mut self, pos: u64) -> bool {
        self.free.insert(pos);
        pos <= self.pos
    }

    fn increment(&mut self) -> u64 {
        let pos = self.pos;
        self.pos += 1;
        pos
    }
}

mod tests {
    #[cfg(test)]
    mod int {
        use super::super::*;

        #[test]
        fn test_generate() {
            let mut generator = IntUid::new();

            assert_eq!(generator.next(), 0);
            assert_eq!(generator.next(), 1);
        }

        #[test]
        fn test_drop() {
            let mut generator = IntUid::new();

            // Fill generator with 3 unique integers
            generator.next();
            generator.next();
            generator.next();

            // Free the middle integer
            generator.drop(2);

            // Next free integer should be 2, not 4 because it is the smallest avaiable integer.
            assert_eq!(generator.next(), 2);
        }
    }
}
