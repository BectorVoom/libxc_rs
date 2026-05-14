//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 827/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk827<F: Float>(t13261: F, t812: F, t2638: F, t4166: F, t2629: F, t820: F, t9645: F, t2696: F, t1516: F, t9601: F, t68: F, t9971: F, t226: F, t1519: F, t2627: F, t1543: F, t2841: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13262 = t812 * t13261;
    let t13278 = t4166 * t2638;
    let t13283 = t4166 * t2629;
    let t13350 = t9645 * t820;
    let t13360 = t4166 * t2696;
    let t13368 = t9601 * t1516;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13416 = t2627 * t1519;
    let t13520 = t1543 * t2841;
    (t13262, t13278, t13283, t13350, t13360, t13368, t13397, t13416, t13520)
}
