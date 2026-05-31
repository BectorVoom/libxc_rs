//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1200/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1200<F: Float>(t1388: F, t3914: F, t1307: F, t3698: F, t3719: F, t1395: F, t2319: F, t1983: F, t23857: F, t6996: F, t22579: F, t6876: F) -> (F, F, F, F, F, F) {
    let t55173 = t3914 * t1388;
    let t55183 = t3698 * t1307;
    let t55246 = t1388 * t3719;
    let t55344 = t1395 * t2319;
    let t80609 = F::cast_from(6.0_f64) * t1983 * t6996 * t23857;
    let t80611 = F::cast_from(3.0_f64) * t6876 * t22579;
    (t55173, t55183, t55246, t55344, t80609, t80611)
}
