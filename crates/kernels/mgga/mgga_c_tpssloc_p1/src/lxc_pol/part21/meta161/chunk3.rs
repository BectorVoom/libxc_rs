//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1052/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1052<F: Float>(t17: F, t3826: F, t1285: F, t592: F, t1287: F) -> (F, F, F, F, F) {
    let t3827 = t17 * t3826;
    let t3828 = F::new(2.0) * t3827;
    let t3829 = t592 * t1285;
    let t3830 = F::new(8.0) * t3829;
    let t3832 = F::new(8.0) * t592 * t1287;
    (t3827, t3828, t3829, t3830, t3832)
}
