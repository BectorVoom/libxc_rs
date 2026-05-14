//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 830/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk830<F: Float>(t13895: F, t973: F, t1599: F, t698: F, t10508: F, t1616: F, t248: F, t1020: F, t122: F, t247: F) -> (F, F, F, F, F) {
    let t13896 = t973 * t13895;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13965 = t248 * t10508 * t1616;
    let t13966 = t1020 * t13965;
    let t13969 = t247 * t122;
    (t13896, t13909, t13965, t13966, t13969)
}
