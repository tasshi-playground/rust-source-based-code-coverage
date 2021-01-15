use std::fmt::Debug;

#[derive(Debug)]
pub struct DebugNotCoveraged {
    pub msg: String,
}

#[derive(Debug)]
pub struct DebugCoveraged {
    pub msg: String,
}

fn main() {
    let debug_not_coveraged = DebugNotCoveraged {
        msg: String::from("Debug is not coveraged"),
    };

    let debug_coveraged = DebugCoveraged {
        msg: String::from("Debug is coveraged"),
    };

    print!("{:?}", debug_coveraged);

    assert!(debug_not_coveraged.msg == "Debug is not coveraged");
}
