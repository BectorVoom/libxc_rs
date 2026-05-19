//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 859/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk859<F: Float>(t21: F, t59: F, t207: F, t795: F, t2690: F, t841: F, t812: F, t849: F, t241: F, t6589: F, t67: F, t2632: F, t776: F) -> (F, F, F, F, F, F) {
    let t9580 = t59 * t21;
    let t9583 = F::cast_from(0.16435185185185185185e-1_f64) * t9580 * t207 * t795;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9602 = t9601 * t849;
    let t9607 = t241 * t6589 * t67;
    let t9627 = t2632 * t776;
    (t9580, t9583, t9601, t9602, t9607, t9627)
}
