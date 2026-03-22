mod compare;
mod dataset;
mod oracle_ffi;
mod report;
mod thresholds;

fn main() {
    println!(
        "verify placeholder: {}",
        vec![
            dataset::describe(),
            oracle_ffi::describe(),
            compare::describe(),
            report::describe(),
            thresholds::describe()
        ]
        .join(" | ")
    );
}
