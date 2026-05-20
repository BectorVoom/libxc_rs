//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2305/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2305<F: Float>(t16052: F, t1992: F, t22897: F, t26392: F, t80670: F, t16419: F, t6976: F, t22705: F, t26422: F, t81228: F, t16040: F, t22633: F, t3807: F) -> (F, F, F, F, F) {
    let t90835 = t1992 * t22897 * t16052;
    let t90837 = t80670 * t26392;
    let t90840 = t1992 * t6976 * t16419;
    let t90844 = t81228 * t22705 * t26422;
    let t90845 = F::cast_from(0.16449340668482264365e-1_f64) * t90844;
    let t90848 = t22633 * t6976 * t16040 * t3807;
    (t90835, t90837, t90840, t90845, t90848)
}
