//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 745/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk745<F: Float>(t2053: F, t2718: F, t4300: F, t13463: F, t1528: F, t2054: F, t23207: F, t23209: F, t23233: F, t23236: F, t24291: F, t24305: F, t25194: F, t2713: F, t4147: F, t4268: F, t4301: F, t7087: F, t7092: F, t7107: F, t7842: F, t855: F) -> (F, F) {
    let t26690 = t2718 * t2053 * t4300;
    let t26698 = 2.0 * t4147 * t7092 + t23207 + 0.82246703342411321825e-2 * t23209 - t2713 * t7842 + 2.0 * t855 * t26690 - t13463 * t2054 - t7087 * t4301 - t24291 + t23233 + 0.3289868133696452873e-1 * t25194 + t23236 - t24305 * t1528 - t4268 * t7107;
    (t26690, t26698)
}
