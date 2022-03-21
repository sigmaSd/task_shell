use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Could not spawn tokio runtime"));

#[no_mangle]
pub fn execute(input: *const u8, len: usize) -> i32 {
    let result = (|| -> Result<i32, Box<dyn std::error::Error>> {
        let input = unsafe { std::slice::from_raw_parts(input, len) };
        let input = std::str::from_utf8(input)?;

        Ok(RUNTIME.block_on(deno_task_shell::execute(
            deno_task_shell::parser::parse(input)?,
            std::env::vars().collect(),
            &std::env::current_dir()?,
        )))
    })();
    match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}
