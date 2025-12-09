use aoc_utils_proc_macro::todo_to_optional_result;

#[todo_to_optional_result]
fn simple_return() -> u32 {
    42
}

#[todo_to_optional_result]
fn simple_operation() -> u32 {
    let x = 5;
    x + 3
}

#[todo_to_optional_result]
fn test_todo() -> u64 {
    todo!()
}

#[todo_to_optional_result]
fn test_todo2() -> u64 {
    let _x = 33;
    todo!()
}

#[todo_to_optional_result]
fn test_todo3() -> u64 {
    let sum = [1, 2, 3].iter().sum();
    todo!()
}

#[todo_to_optional_result]
fn test_todo_in_iter() -> u64 {
    [1, 2, 3].iter().map(|x| todo!()).sum()
}