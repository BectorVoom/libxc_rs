//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 918/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk918<F: Float>(t8577: F, t9153: F, t39208: F, t8457: F, t1907: F, t1971: F, t209: F, t236: F, t476: F, t7453: F, t2283: F, t38351: F) -> (F, F, F, F) {
    let t45274 = t8577 * t9153;
    let t45277 = t39208 * t8457;
    let t45283 = t7453 * t1971 * t236 * t1907 * t476 * t209;
    let t45285 = t38351 * t2283;
    (t45274, t45277, t45283, t45285)
}
