//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 942/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk942<F: Float>(t1068: F, t2973: F, t1072: F, t2966: F, t2998: F, t425: F, t9347: F, t9172: F, t2993: F, t3001: F, t9213: F, t1053: F, t2922: F) -> (F, F, F, F, F, F, F, F) {
    let t9359 = t1068 * t2973;
    let t9365 = t2966 * t1072;
    let t9370 = t1068 * t2998;
    let t9373 = t425 * t9347;
    let t9380 = t425 * t9172;
    let t9384 = t2993 * t3001;
    let t9399 = F::new(0.55403703703703703703e-1) * t9213;
    let t9414 = t2922 * t1053;
    (t9359, t9365, t9370, t9373, t9380, t9384, t9399, t9414)
}
