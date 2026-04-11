mod generate_dispatch;
mod generate_registry;
mod parse_functionals;
mod parse_xc_h;

fn main() {
    let tasks = [
        parse_xc_h::describe(),
        parse_functionals::describe(),
        generate_registry::describe(),
        generate_dispatch::describe(),
    ];
    let summary = tasks.iter().copied().collect::<Vec<_>>().join(" | ");
    println!("libxc_rs xtask placeholder tasks: {}", summary);
}
