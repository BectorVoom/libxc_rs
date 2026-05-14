//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 845/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk845<F: Float>(t23046: F, t5585: F, t6605: F, t1894: F, t23078: F, t5527: F, t59: F, t5624: F, t8343: F, t23097: F, t28395: F, t6612: F, t1516: F, t32840: F, t5628: F, t1880: F, t25224: F, t32866: F) -> (F, F, F, F, F, F, F) {
    let t126328 = t6605 * t23046 * t5585;
    let t126332 = t23078 * t1894 * t59 * t5527;
    let t126334 = t8343 * t5624;
    let t126337 = t23097 * t6612 * t28395;
    let t126339 = t32840 * t1516;
    let t126341 = t8343 * t5628;
    let t126349 = 0.3289868133696452873e-1 * t1880 * t25224 * t32866;
    (t126328, t126332, t126334, t126337, t126339, t126341, t126349)
}
