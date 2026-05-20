//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2022/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2022<F: Float>(t80780: F, t91206: F, t91221: F, t91223: F, t93674: F, t93682: F, t97310: F, t97315: F, t97318: F, t97320: F, t97322: F, t97326: F, t97333: F, t97337: F, t97340: F, t97342: F, t97344: F, t97347: F) -> F {
    let t102694 = -t97310 / F::new(48.0) + F::cast_from(0.67287926823567318088e-4_f64) * t97315 + t97318 / F::new(768.0) + t97320 / F::new(192.0) + t97322 / F::new(96.0) - F::cast_from(0.40372756094140390853e-3_f64) * t97326 - F::cast_from(0.126501302428306558e-1_f64) * t91206 - t93674 - t91221 - t91223 + t93682 - F::cast_from(0.24223653656484234512e-2_f64) * t97333 + F::cast_from(0.80745512188280781706e-3_f64) * t97337 - F::cast_from(0.63250651214153279004e-2_f64) * t80780 - t97340 / F::new(192.0) - t97342 / F::new(96.0) - t97344 / F::new(96.0) - F::cast_from(0.80745512188280781707e-3_f64) * t97347;
    t102694
}
