#[no_mangle]
pub fn execute(input: *const u8, len: usize) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let input = std::str::from_utf8(input).unwrap();

    rt.block_on(deno_task_shell::execute(
        deno_task_shell::parser::parse(input).unwrap(),
        std::env::vars().collect(),
        &std::env::current_dir().unwrap(),
    ));
}
