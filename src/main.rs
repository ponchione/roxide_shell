use shell::roxide_shell;

mod shell;

fn main() {
    print_logo();
    roxide_shell::shell();
}

fn print_logo() {
    println!("  ______  _____  _     _ _____ ______  _______");
    println!(" |_____/ |     |  \\___/    |   |     \\ |______");
    println!(" |    \\_ |_____| _/   \\_ __|__ |_____/ |______    v0.1.0\n");
}