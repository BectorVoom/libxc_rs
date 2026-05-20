//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 576/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk576<F: Float>(t17: F, t3824: F, t1287: F, t592: F, t588: F, t1365: F, t68: F, t248: F, t2691: F, t557: F, t555: F, t1361: F, t835: F) -> (F, F, F, F, F, F, F) {
    let t3825 = t17 * t3824;
    let t3832 = F::new(8.0) * t592 * t1287;
    let t3836 = F::new(8.0) * t588 * t1287;
    let t3843 = t68 * t1365;
    let t3862 = t2691 * t557 * t248;
    let t3864 = F::new(119.0) / F::new(13824.0) * t555 * t3862;
    let t3865 = t1361 * t835;
    (t3825, t3832, t3836, t3843, t3862, t3864, t3865)
}
