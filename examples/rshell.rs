/**
 * 【译】使用 Rust 构建你自己的 Shell
 *  https://www.cnblogs.com/ishenghuo/p/12550142.html
 * 
 * shell项目1: https://github.com/JoshMcguigan/bubble-shell
 * shell项目2: https://github.com/psinghal20/rush
 * 
 */
use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        // 必须是可以 peek 的，这样我们才能确定何时结束
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "exit" | "quit"=> return, // shell内建功能： 退出shell命令
                "cd" => { // shell内建功能： cd命令
                    // 如果没有提供路径参数，则默认 '/' 路径
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                command => {
                    // 管道符:
                    // 可以使用 | 字符告诉 shell 将第一个命令的结果输出重定向到第二个命令的输入。例如，运行 ls | grep Cargo
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        // 在这个命令后还有另一个命令，准备将其输出到下一个命令
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        // 在发送输出到 shell 的 stdout 之后，就没有命令要执行了
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            // 阻塞一直到命令执行完成find . -type d -exec cd {} \;
            final_command.wait().unwrap();
        }
    }
}
