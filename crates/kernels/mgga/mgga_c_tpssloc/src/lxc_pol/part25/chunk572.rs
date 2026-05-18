//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 572/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk572<F: Float>(t2244: F, t3555: F, t974: F, t3242: F, t3439: F, t225: F, t3481: F, t68: F, t484: F, t121: F, t486: F, t1216: F, t248: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3556 = t3555 * t2244;
    let t3557 = t974 * t3556;
    let t3560 = t3439 * t3242;
    let t3561 = t3560 * t2244;
    let t3562 = t974 * t3561;
    let t3565 = t3481 * t225;
    let t3566 = t3565 * t68;
    let t3567 = t3566 * t484;
    let t3570 = t121 * t486;
    let t3572 = t248 * t3570 * t1216;
    (t3556, t3557, t3561, t3562, t3565, t3566, t3567, t3570, t3572)
}
