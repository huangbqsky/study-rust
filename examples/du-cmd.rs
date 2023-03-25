use std::process::{Command, Stdio};
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
    }
}
/**
 * 下面的例子将显示当前目录中大小排名前十的文件和子目录，
 * 效果等效于命令 du -ah . | sort -hr | head -n 10
 */
fn main() -> Result<()> {
    let directory = std::env::current_dir()?;
    // du -ah .
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;

    // | sort -hr 
    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()?;
        du_output_child.wait()?;

        // | head -n 10
        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child = Command::new("head")
                .args(&["-n", "10"])
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()?;
            let head_stdout = head_output_child.wait_with_output()?;
            sort_output_child.wait()?;

            println!("Top 10 bigggest files and directories in ‘{}’:\n{}",
                     directory.display(),
                     String::from_utf8(head_stdout.stdout).unwrap()
            );
        }

    }
    Ok(())
}