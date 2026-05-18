//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 673/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk673<F: Float>(t3198: F, t1173: F, t1184: F, t1268: F) -> (F, F, F, F) {
    let t3199 = F::new(2.0) * t3198;
    let t3200 = t1173 * t1184;
    let t3201 = F::new(8.0) * t3200;
    let t3202 = t1268 * t1268;
    (t3199, t3200, t3201, t3202)
}
