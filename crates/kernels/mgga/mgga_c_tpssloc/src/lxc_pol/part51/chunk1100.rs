//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1100/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1100<F: Float>(t1799: F, t31549: F, t22635: F, t22633: F, t31618: F, t6637: F, t6888: F, t27074: F, t550: F, t6976: F, t1992: F, t1998: F, t7918: F, t214: F, t1985: F, t1825: F, t31636: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33272 = t31549 * t1799;
    let t33273 = t22635 * t33272;
    let t33274 = t22633 * t33273;
    let t33276 = t31618 * t1799;
    let t33277 = t6637 * t33276;
    let t33278 = t6888 * t33277;
    let t33280 = t27074 * t550;
    let t33281 = t6976 * t33280;
    let t33282 = t1992 * t33281;
    let t33284 = t1998 * t7918;
    let t33285 = t214 * t33284;
    let t33286 = t1985 * t33285;
    let t33289 = t31636 * t1825;
    (t33272, t33273, t33274, t33276, t33277, t33278, t33280, t33281, t33282, t33284, t33285, t33286, t33289)
}
