//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 426/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk426<F: Float>(t221: F, t2966: F, t339: F, t135: F, t976: F, t979: F, t973: F, t986: F, t271: F, t883: F, t974: F, t2770: F, t344: F) -> (F, F, F, F, F, F) {
    let t2967 = t221 * t2966;
    let t2969 = F::new(0.18518518518518518518e-3) * t339 * t2967;
    let t2970 = t135 * t976;
    let t2971 = t2970 * t979;
    let t2972 = t973 * t2971;
    let t2974 = t135 * t986;
    let t2975 = t973 * t2974;
    let t2978 = F::new(1.0) / t271 / t883;
    let t2979 = t974 * t2978;
    let t2980 = t344 * t2770;
    (t2969, t2972, t2975, t2978, t2979, t2980)
}
