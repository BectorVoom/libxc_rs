//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 440/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk440<F: Float>(t1893: F, t1895: F, t235: F, t59: F, t226: F, t249: F, t1888: F) -> (F, F, F) {
    let t1896 = t1893 * t1895;
    let t1898 = t235 * t59;
    let t1899 = t226 * t1898;
    let t1900 = t1899 * t249;
    let t1902 = t1888 / 96.0 + 0.20186378047070195427e-3 * t1896 + t1900 / 1536.0;
    (t1898, t1899, t1902)
}
