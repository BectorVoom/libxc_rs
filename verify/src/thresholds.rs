/// Tolerance constants for oracle comparison, derived from project accuracy requirements.
///
/// Each constant represents the maximum acceptable relative error (using the
/// metric in compare::relative_error) for the corresponding derivative order.

/// Energy (exc) tolerance: relative error <= 10^-12 (ACC-01, D-02)
pub const EXC_TOLERANCE: f64 = 1e-12;

/// First derivative (vxc/vrho) tolerance: relative error <= 10^-10 (ACC-02, D-02)
pub const VXC_TOLERANCE: f64 = 1e-10;

/// Second derivative (fxc) tolerance: relative error <= 10^-8 (ACC-03)
pub const FXC_TOLERANCE: f64 = 1e-8;

/// Third derivative (kxc) tolerance: relative error <= 10^-6 (ACC-04)
pub const KXC_TOLERANCE: f64 = 1e-6;

/// Fourth derivative (lxc) tolerance: relative error <= 10^-4 (ACC-05)
pub const LXC_TOLERANCE: f64 = 1e-4;

/// Returns the tolerance for a given derivative order (0-4).
///
/// Order 0 = exc, 1 = vxc, 2 = fxc, 3 = kxc, 4 = lxc.
pub fn tolerance_for_order(order: usize) -> f64 {
    match order {
        0 => EXC_TOLERANCE,
        1 => VXC_TOLERANCE,
        2 => FXC_TOLERANCE,
        3 => KXC_TOLERANCE,
        4 => LXC_TOLERANCE,
        _ => panic!("derivative order {order} out of range 0..=4"),
    }
}
