use is_terminal::IsTerminal;

fn main() {
    println!("stdin? {}", std::io::stdin().is_terminal());
    println!("stdout? {}", std::io::stdout().is_terminal());
    println!("stderr? {}", std::io::stderr().is_terminal());
}
