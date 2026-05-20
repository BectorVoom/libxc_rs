//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1481/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1481<F: Float>(t210: F, t4158: F, t776: F, t1495: F, t2553: F, t120: F, t4119: F, t2645: F, t829: F, t2679: F, t4248: F, t13242: F, t4180: F) -> (F, F, F, F, F) {
    let t13293 = t210 * t4158 * t776;
    let t13297 = t210 * t1495 * t2553;
    let t13300 = t120 * t4119;
    let t13302 = t2645 * t13300 * t829;
    let t13306 = t2645 * t4248 * t2679;
    let t13312 = t4180 * t13242 * t829;
    (t13293, t13297, t13302, t13306, t13312)
}
