//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2294/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2294<F: Float>(t1933: F, t23479: F, t99660: F, t1015: F, t28581: F, t82895: F, t28577: F, t3128: F, t25641: F, t88451: F, t1615: F, t17157: F, t17167: F, t17171: F, t1920: F, t25679: F, t25683: F, t2987: F, t363: F, t4509: F, t6800: F, t88351: F, t88354: F, t88372: F, t88430: F, t88431: F, t88704: F) -> F {
    let t99796 = t1933 * t99660 * t23479;
    let t99799 = t82895 * t1015 * t28581;
    let t99802 = t82895 * t3128 * t28577;
    let t99813 = t88451 * t25641;
    let t99826 = t88704 - F::cast_from(0.20186378047070195428e-3_f64) * t99796 - F::cast_from(0.10093189023535097714e-3_f64) * t99799 + F::cast_from(0.20186378047070195428e-3_f64) * t99802 - F::cast_from(0.20186378047070195428e-3_f64) * t88430 * t88431 * t363 * t1615 * t6800 - F::cast_from(0.40372756094140390856e-3_f64) * t88372 * t88351 + F::cast_from(0.20186378047070195428e-3_f64) * t88372 * t88354 + F::cast_from(0.20186378047070195428e-3_f64) * t99813 - t1920 * t2987 * t17171 / F::new(72.0) - t1920 * t4509 * t17157 / F::new(36.0) + t1920 * t2987 * t17167 / F::new(48.0) + F::cast_from(0.20186378047070195428e-3_f64) * t25683 * t25679;
    t99826
}
