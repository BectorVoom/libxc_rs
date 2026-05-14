//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 924/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk924<F: Float>(t425: F, t9172: F, t9213: F, t1049: F, t2954: F, t2953: F, t417: F, t412: F, t9181: F, t1052: F, t2956: F, t420: F, t2929: F, t1022: F, t2909: F, t394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9380 = t425 * t9172;
    let t9399 = 0.55403703703703703703e-1 * t9213;
    let t9419 = t1049 * t2954;
    let t9423 = 1.0 / t2953 / t417;
    let t9424 = t412 * t9423;
    let t9429 = 0.46308888888888888888e0 * t9181;
    let t9438 = 0.16068111111111111111e1 * t9213;
    let t9464 = 1.0 / t2953 / t1052;
    let t9465 = t412 * t9464;
    let t9467 = 1.0 / t2956 / t420;
    let t9471 = t1049 * t2929;
    let t9477 = 0.53272592592592592592e-1 * t9213;
    let t9492 = 1.0 / t2909 / t1022;
    let t9493 = t394 * t9492;
    (t9380, t9399, t9419, t9424, t9429, t9438, t9465, t9467, t9471, t9477, t9493)
}
