//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 760/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk760<F: Float>(t2570: F, t782: F, t2690: F, t59: F, t154: F, t2588: F, t21: F, t207: F, t795: F, t841: F, t812: F, t241: F, t6589: F, t67: F, t815: F, t836: F) -> (F, F, F, F, F, F, F, F) {
    let t9573 = t782 * t2570;
    let t9576 = t59 * t2690;
    let t9577 = t9576 * t154;
    let t9579 = 0.99999999999999999997e-2 * t9577 * t2588;
    let t9580 = t59 * t21;
    let t9583 = 0.16435185185185185185e-1 * t9580 * t207 * t795;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9607 = t241 * t6589 * t67;
    let t9637 = t815 * t836;
    (t9573, t9577, t9579, t9580, t9583, t9601, t9607, t9637)
}
