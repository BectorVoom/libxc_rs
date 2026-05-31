//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1119/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1119<F: Float>(t15248: F, t15251: F, t15292: F, t15294: F, t15296: F, t15299: F, t15301: F, t15304: F, t15307: F, t15309: F, t15312: F, t11938: F, t11958: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t15334: F, t15339: F, t15342: F) -> (F, F) {
    let t15385 = -F::cast_from(0.54771111111111111111e-1_f64) * t15248 + F::cast_from(0.29896666666666666667e0_f64) * t15251 + F::cast_from(0.1898925e1_f64) * t15292 + F::cast_from(0.3071625e0_f64) * t15294 + F::cast_from(0.18257037037037037037e-1_f64) * t15296 - F::cast_from(0.76790625e-1_f64) * t15299 + F::cast_from(0.3071625e0_f64) * t15301 + F::cast_from(0.15358125e0_f64) * t15304 + F::cast_from(0.142419375e1_f64) * t15307 - F::cast_from(0.1898925e1_f64) * t15309 - F::cast_from(0.9494625e0_f64) * t15312;
    let t15406 = -F::cast_from(0.16431333333333333333e0_f64) * t15334 + F::cast_from(0.26574814814814814815e0_f64) * t11938 - t11958 - F::cast_from(0.19931111111111111111e0_f64) * t15283 + F::cast_from(0.36514074074074074075e-1_f64) * t15339 - F::cast_from(0.27385555555555555556e-1_f64) * t15342 - F::cast_from(0.39862222222222222222e0_f64) * t15268 - F::cast_from(0.11958666666666666667e1_f64) * t15264 + F::cast_from(0.11958666666666666667e1_f64) * t15277 + F::cast_from(0.17938e1_f64) * t15273 + F::cast_from(0.59793333333333333334e0_f64) * t15288;
    (t15385, t15406)
}
