//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1816/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1816<F: Float>(t10599: F, t1547: F, t2799: F, t13615: F, t894: F, t1553: F, t2403: F) -> (F, F, F, F) {
    let t13637 = t10599 * t1547;
    let t13638 = t13637 * t2799;
    let t13640 = t894 * t13615;
    let t13642 = t2403 * t1553;
    (t13637, t13638, t13640, t13642)
}
