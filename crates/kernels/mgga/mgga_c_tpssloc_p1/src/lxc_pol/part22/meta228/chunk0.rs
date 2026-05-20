//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1289/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1289<F: Float>(t2588: F, t9577: F, t21: F, t59: F, t207: F, t795: F, t2690: F, t841: F, t812: F) -> (F, F, F, F, F) {
    let t9579 = F::cast_from(0.99999999999999999997e-2_f64) * t9577 * t2588;
    let t9580 = t59 * t21;
    let t9583 = F::cast_from(0.16435185185185185185e-1_f64) * t9580 * t207 * t795;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    (t9579, t9580, t9583, t9600, t9601)
}
