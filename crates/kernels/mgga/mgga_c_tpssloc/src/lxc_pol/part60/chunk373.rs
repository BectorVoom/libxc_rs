//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 373/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk373<F: Float>(t2928: F, t315: F, t323: F, t340: F, t697: F, t344: F, t221: F, t339: F, t135: F, t976: F, t271: F, t883: F, t974: F, t2770: F, t337: F, t39: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2929 = 1.0 / t2928;
    let t2930 = t315 * t2929;
    let t2931 = t323 * t323;
    let t2932 = 1.0 / t2931;
    let t2965 = t697 * t340;
    let t2966 = t2965 * t344;
    let t2967 = t221 * t2966;
    let t2969 = 0.18518518518518518518e-3 * t339 * t2967;
    let t2970 = t135 * t976;
    let t2978 = 1.0 / t271 / t883;
    let t2979 = t974 * t2978;
    let t2980 = t344 * t2770;
    let t2985 = t39 * t337;
    (t2929, t2930, t2932, t2965, t2969, t2970, t2978, t2979, t2980, t2985)
}
