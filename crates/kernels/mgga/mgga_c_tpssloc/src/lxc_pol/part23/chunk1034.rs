//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1034/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1034<F: Float>(t3684: F, t39354: F, t181: F, t2558: F, t686: F, t1291: F, t2369: F, t9720: F, t9843: F, t1294: F, t2411: F, t2414: F, t39246: F) -> (F, F, F, F, F, F) {
    let t39356 = 0.21687162600603479684e-1 * t3684 * t39354;
    let t39358 = t686 * t2558 * t181;
    let t39360 = 0.18989649058080861537e-2 * t1291 * t39358;
    let t39362 = t9720 * t2369 * t9843;
    let t39364 = 0.62337092780453269531e3 * t1294 * t39362;
    let t39373 = 0.48245938496077605201e2 * t2411 * t39246 * t2414;
    (t39356, t39358, t39360, t39362, t39364, t39373)
}
