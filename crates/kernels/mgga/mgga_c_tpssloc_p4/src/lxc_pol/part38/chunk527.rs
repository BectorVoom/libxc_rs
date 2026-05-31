//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 527/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk527<F: Float>(t2250: F, t31: F, t65: F, t608: F, t628: F, t36: F, t365: F, t42: F, t2244: F, t43: F, t54: F, t55: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2251 = t31 * t2250;
    let t2252 = t2251 * t65;
    let t2255 = t608 * t628;
    let t2261 = F::cast_from(1.0_f64) / t36 / t365;
    let t2262 = sigma0 * t2261;
    let t2267 = F::cast_from(1.0_f64) / t42;
    let t2268 = t2267 * t2244;
    let t2271 = t43 * t2250;
    let t2274 = F::cast_from(1.0_f64) / t54;
    let t2275 = t2274 * t2244;
    let t2278 = t55 * t2250;
    (t2251, t2252, t2255, t2262, t2267, t2268, t2271, t2274, t2275, t2278)
}
