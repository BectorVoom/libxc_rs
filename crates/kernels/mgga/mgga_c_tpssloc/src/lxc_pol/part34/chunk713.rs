//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 713/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk713<F: Float>(t2638: F, t4166: F, t2629: F, t2696: F, t1516: F, t9601: F, t1519: F, t2627: F, t1543: F, t2841: F, t1540: F, t2394: F) -> (F, F, F, F, F, F, F) {
    let t13278 = t4166 * t2638;
    let t13283 = t4166 * t2629;
    let t13360 = t4166 * t2696;
    let t13368 = t9601 * t1516;
    let t13416 = t2627 * t1519;
    let t13520 = t1543 * t2841;
    let t13598 = t2394 * t1540;
    (t13278, t13283, t13360, t13368, t13416, t13520, t13598)
}
