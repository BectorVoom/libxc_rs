//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2224/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2224<F: Float>(t17349: F, t2888: F, t17191: F, t300: F, t18169: F, t3216: F, t11094: F, t5946: F, t17297: F, t2929: F, t18065: F, t225: F) -> (F, F, F, F, F, F) {
    let t60775 = t17349 * t2888;
    let t60848 = t300 * t17191;
    let t60867 = t18169 * t3216;
    let t60874 = t5946 * t11094;
    let t60963 = t2929 * t17297;
    let t60971 = t18065 * t225;
    (t60775, t60848, t60867, t60874, t60963, t60971)
}
