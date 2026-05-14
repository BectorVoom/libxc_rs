//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 885/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk885<F: Float>(t1441: F, t7467: F, t2040: F, t33211: F, t7796: F, t102386: F, t1874: F, t28239: F, t8607: F, t22574: F, t28830: F, t36740: F, t31682: F, t5398: F, t8308: F, t113875: F, t121022: F, t1433: F) -> (F, F, F, F, F, F, F, F) {
    let t128296 = t1441 * t7467;
    let t128298 = 4.0 * t128296 * t2040;
    let t128300 = 4.0 * t33211 * t7796;
    let t128302 = 2.0 * t102386 * t1874;
    let t128303 = t8607 * t28239;
    let t128306 = 6.0 * t22574 * t36740 * t28830;
    let t128311 = t8308 * t31682 * t5398;
    let t128317 = t113875 * t121022 * t1433;
    (t128296, t128298, t128300, t128302, t128303, t128306, t128311, t128317)
}
