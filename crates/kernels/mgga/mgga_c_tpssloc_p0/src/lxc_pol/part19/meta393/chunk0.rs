//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1491/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1491<F: Float>(t626: F, t9412: F, t106: F, t9364: F, t2332: F, t2358: F, t2248: F, t35761: F, t2350: F, t2354: F, t39108: F, t35577: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t45432 = t626 * t9412;
    let t45435 = F::new(1.0) / t9364 / t106;
    let t45436 = t2332 * t2332;
    let t45444 = t2358 * t2358;
    let t45453 = t2248 * t2248;
    let t45460 = F::new(1.0) / t35761;
    let t45461 = t2350 * t2350;
    let t45469 = t2354 * t2354;
    let t45482 = F::new(12.0) * t39108;
    let t45496 = F::new(1.0) / t35577;
    (t45432, t45435, t45436, t45444, t45453, t45460, t45461, t45469, t45482, t45496)
}
