//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 725/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk725<F: Float>(t553: F, t6955: F, t1332: F, t1336: F, t2013: F, t544: F, t6967: F, t6971: F, t6975: F, t6980: F, t6984: F, t6988: F) -> (F, F) {
    let t6990 = t553 * t6955;
    let t6992 = -t6967 - F::cast_from(0.16449340668482264365e-1_f64) * t6971 - t6975 - F::cast_from(0.82246703342411321825e-2_f64) * t6980 + F::cast_from(0.82246703342411321825e-2_f64) * t6984 + t1332 * t2013 - t1336 * t6988 + t544 * t6990;
    (t6990, t6992)
}
