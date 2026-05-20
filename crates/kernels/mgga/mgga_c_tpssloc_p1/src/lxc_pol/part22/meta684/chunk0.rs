//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2250/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2250<F: Float>(t1036: F, t18010: F, t14025: F, t14508: F, t13970: F, t14511: F, t1057: F, t61729: F, t3199: F, t61734: F, t3185: F, t18053: F, t225: F) -> (F, F, F, F, F, F, F) {
    let t62893 = t18010 * t1036;
    let t62901 = t14508 * t14025;
    let t62903 = t14511 * t13970;
    let t62994 = t61729 * t1057;
    let t63004 = t61734 * t3199;
    let t63183 = t61734 * t3185;
    let t63215 = t18053 * t225;
    (t62893, t62901, t62903, t62994, t63004, t63183, t63215)
}
