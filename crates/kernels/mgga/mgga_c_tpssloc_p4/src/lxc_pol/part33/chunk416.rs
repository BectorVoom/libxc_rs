//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 416/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk416<F: Float>(t1893: F, t1895: F, t235: F, t59: F, t226: F, t249: F, t1888: F) -> (F, F, F) {
    let t1896 = t1893 * t1895;
    let t1898 = t235 * t59;
    let t1899 = t226 * t1898;
    let t1900 = t1899 * t249;
    let t1902 = t1888 / F::cast_from(96.0_f64) + F::cast_from(0.20186378047070195427e-3_f64) * t1896 + t1900 / F::cast_from(1536.0_f64);
    (t1898, t1899, t1902)
}
