//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 909/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk909<F: Float>(t9181: F, t9213: F, t1052: F, t2953: F, t412: F, t2956: F, t420: F, t1049: F, t2929: F, t1022: F, t2909: F, t394: F) -> (F, F, F, F, F, F, F) {
    let t9429 = F::cast_from(0.46308888888888888888e0_f64) * t9181;
    let t9438 = F::cast_from(0.16068111111111111111e1_f64) * t9213;
    let t9464 = F::new(1.0) / t2953 / t1052;
    let t9465 = t412 * t9464;
    let t9467 = F::new(1.0) / t2956 / t420;
    let t9471 = t1049 * t2929;
    let t9477 = F::cast_from(0.53272592592592592592e-1_f64) * t9213;
    let t9492 = F::new(1.0) / t2909 / t1022;
    let t9493 = t394 * t9492;
    (t9429, t9438, t9465, t9467, t9471, t9477, t9493)
}
