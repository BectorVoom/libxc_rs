//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 480/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk480<F: Float>(t2967: F, t339: F, t135: F, t976: F, t979: F, t973: F, t986: F, t271: F, t883: F, t974: F, t337: F, t39: F, t1887: F, t60: F) -> (F, F, F, F, F, F, F, F) {
    let t2969 = 0.18518518518518518518e-3 * t339 * t2967;
    let t2970 = t135 * t976;
    let t2971 = t2970 * t979;
    let t2972 = t973 * t2971;
    let t2974 = t135 * t986;
    let t2975 = t973 * t2974;
    let t2978 = 1.0 / t271 / t883;
    let t2979 = t974 * t2978;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    (t2969, t2970, t2972, t2975, t2978, t2979, t2986, t2987)
}
