//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1113/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1113<F: Float>(t11938: F, t11940: F, t11941: F, t11943: F, t15239: F, t15241: F, t15243: F, t15251: F, t15259: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t9221: F, t9243: F) -> F {
    let t15291 = -t9243 + F::new(4.0) / F::new(27.0) * t9221 + F::new(8.0) / F::new(27.0) * t11938 + t11940 - t11941 - t11943 + F::new(2.0) / F::new(27.0) * t15239 + F::new(10.0) / F::new(27.0) * t15259 - F::new(4.0) / F::new(3.0) * t15264 - F::new(4.0) / F::new(9.0) * t15268 - F::new(2.0) / F::new(9.0) * t15241 + F::new(2.0) * t15273 + F::new(4.0) / F::new(3.0) * t15277 - t15243 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t15283 + F::new(2.0) / F::new(3.0) * t15288 + t15251 / F::new(3.0);
    t15291
}
