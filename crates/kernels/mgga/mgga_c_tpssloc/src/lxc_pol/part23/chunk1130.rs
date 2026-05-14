//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1130/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130<F: Float>(t21061: F, t225: F, t21036: F, t20852: F, t252: F, t1519: F, t5611: F, t21013: F, t814: F, t20937: F, t68: F, t20217: F, t707: F, t751: F, t184: F, t120: F, t20856: F) -> (F, F, F, F, F, F, F, F, F) {
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    let t67392 = t252 * t20852;
    let t67405 = t1519 * t5611;
    let t67429 = t814 * t21013;
    let t67441 = t20937 * t68;
    let t67463 = t707 * t751 * t20217;
    let t67469 = t184 * t20217;
    let t67607 = t120 * t20856;
    (t67339, t67344, t67392, t67405, t67429, t67441, t67463, t67469, t67607)
}
