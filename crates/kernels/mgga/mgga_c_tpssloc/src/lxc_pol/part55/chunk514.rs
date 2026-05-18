//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 514/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk514<F: Float>(t3545: F, t456: F, t1197: F, t135: F, t1174: F, t121: F, t486: F, t1216: F, t248: F, t1213: F, t478: F, t483: F) -> (F, F, F, F, F, F, F) {
    let t3547 = t456 * t3545 / F::new(432.0);
    let t3548 = t135 * t1197;
    let t3549 = t1174 * t3548;
    let t3570 = t121 * t486;
    let t3572 = t248 * t3570 * t1216;
    let t3573 = t1213 * t3572;
    let t3575 = t478 * t483;
    (t3547, t3548, t3549, t3570, t3572, t3573, t3575)
}
