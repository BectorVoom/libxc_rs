//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 934/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk934<F: Float>(t1068: F, t2998: F, t425: F, t9347: F, t9172: F, t2993: F, t3001: F, t9213: F, t1053: F, t2922: F, t1049: F, t2954: F, t2953: F, t417: F, t412: F, t9181: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9370 = t1068 * t2998;
    let t9373 = t425 * t9347;
    let t9380 = t425 * t9172;
    let t9384 = t2993 * t3001;
    let t9399 = 0.55403703703703703703e-1 * t9213;
    let t9414 = t2922 * t1053;
    let t9419 = t1049 * t2954;
    let t9423 = 1.0 / t2953 / t417;
    let t9424 = t412 * t9423;
    let t9429 = 0.46308888888888888888e0 * t9181;
    (t9370, t9373, t9380, t9384, t9399, t9414, t9419, t9424, t9429)
}
