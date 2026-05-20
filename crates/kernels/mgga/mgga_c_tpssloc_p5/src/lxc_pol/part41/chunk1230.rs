//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1230/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1230<F: Float>(t25: F, t6320: F, t67: F, t758: F, t12061: F, t6305: F, t3664: F, t5397: F, t16557: F, t2219: F, t5134: F, t514: F, t606: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t19541 = t6320 * t67;
    let t19542 = t19541 * t758;
    let t19543 = F::cast_from(0.18311447306006545054e-3_f64) * t19542;
    let t19547 = t12061 * t6305;
    let t19552 = t3664 * t5397;
    let t19558 = piecewise3::<F>(t26, F::new(0.0), -F::new(8.0) / F::new(27.0) * t19547 * t606 + F::new(16.0) / F::new(9.0) * t5134 * t2219 + F::new(4.0) / F::new(9.0) * t19552 * t606 + F::new(4.0) / F::new(3.0) * t514 * t16557);
    (t19543, t19558)
}
