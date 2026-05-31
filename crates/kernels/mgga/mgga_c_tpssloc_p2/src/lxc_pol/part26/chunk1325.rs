//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1325/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1325<F: Float>(t15904: F, t22574: F, t31035: F, t12303: F, t24995: F, t8945: F, t1266: F, t22479: F, t652: F, t1874: F, t45637: F, t12458: F, t40611: F) -> (F, F, F, F, F) {
    let t83684 = F::cast_from(18.0_f64) * t22574 * t31035 * t15904;
    let t83687 = F::cast_from(18.0_f64) * t24995 * t8945 * t12303;
    let t83692 = F::cast_from(6.0_f64) * t652 * t1266 * t22479;
    let t83694 = F::cast_from(6.0_f64) * t45637 * t1874;
    let t83695 = t40611 * t12458;
    (t83684, t83687, t83692, t83694, t83695)
}
