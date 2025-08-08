use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone)]
pub struct TestData {
    pub value: i32,
}

impl Arbitrary for TestData {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            value: i32::arbitrary(g),
        }
    }
}
