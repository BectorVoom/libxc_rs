//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1417/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1417<F: Float>(t1266: F, t22479: F, t652: F, t1874: F, t45637: F, t12458: F, t40611: F, t1983: F, t2019: F, t2235: F, t2244: F, t71: F, t9338: F) -> (F, F, F, F, F) {
    let t83692 = F::cast_from(6.0_f64) * t652 * t1266 * t22479;
    let t83694 = F::cast_from(6.0_f64) * t45637 * t1874;
    let t83695 = t40611 * t12458;
    let t83698 = F::cast_from(6.0_f64) * t1983 * t2019 * t83695;
    let t83699 = t2235 * t2244;
    let t83706 = t71 * t9338;
    (t83692, t83694, t83698, t83699, t83706)
}
