//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 908/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk908<F: Float>(t1068: F, t2973: F, t2998: F, t425: F, t9347: F, t9172: F, t9213: F, t1049: F, t2954: F, t2953: F, t417: F, t412: F) -> (F, F, F, F, F, F, F) {
    let t9359 = t1068 * t2973;
    let t9370 = t1068 * t2998;
    let t9373 = t425 * t9347;
    let t9380 = t425 * t9172;
    let t9399 = F::cast_from(0.55403703703703703703e-1_f64) * t9213;
    let t9419 = t1049 * t2954;
    let t9423 = F::cast_from(1.0_f64) / t2953 / t417;
    let t9424 = t412 * t9423;
    (t9359, t9370, t9373, t9380, t9399, t9419, t9424)
}
