
// task: can I impl a trait twice
// the trait have associated type, and I impl with different associated type
//
use std::fmt::Debug;

trait Label {
    type ToDisplay: Debug;
    fn to_label() -> Self::ToDisplay;
}

struct Person;

impl Label for Person {
    type ToDisplay = String;
    fn to_label() -> Self::ToDisplay {
        "Person".to_string()
    }
}

// error here: conflicting implementation of trait `Label` for type `Person`:

// impl Label for Person {
//     type ToDisplay = i32;
//     fn to_label() -> Self::ToDisplay {
//         0
//     }
// }
//

// conclusion: so the conculsion here is that we can't impl a trait twice
// even if the trait have associated type, and we impl with different associated type
