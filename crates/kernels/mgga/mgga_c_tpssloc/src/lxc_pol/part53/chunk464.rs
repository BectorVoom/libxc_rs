//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 464/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk464<F: Float>(t17: F, t3824: F, t1284: F, t750: F, t1285: F, t592: F, t1287: F, t588: F, t248: F, t2691: F, t557: F, t555: F) -> (F, F, F, F, F, F, F) {
    let t3825 = t17 * t3824;
    let t3826 = t1284 * t750;
    let t3827 = t17 * t3826;
    let t3829 = t592 * t1285;
    let t3832 = F::new(8.0) * t592 * t1287;
    let t3833 = t588 * t1285;
    let t3862 = t2691 * t557 * t248;
    let t3864 = F::new(119.0) / F::new(13824.0) * t555 * t3862;
    (t3825, t3827, t3829, t3832, t3833, t3862, t3864)
}
