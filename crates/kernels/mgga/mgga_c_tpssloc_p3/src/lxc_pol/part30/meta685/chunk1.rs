//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2162/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2162<F: Float>(t1831: F, t91191: F, t26257: F, t5314: F, t28100: F, t80853: F, t80855: F, t80767: F, t80776: F, t80780: F, t91206: F, t91215: F, t91226: F, t97310: F, t97315: F, t97318: F, t97320: F, t97322: F, t97326: F, t97333: F, t97337: F, t97340: F) -> F {
    let t97342 = t91191 * t1831;
    let t97344 = t26257 * t5314;
    let t97347 = t80853 * t80855 * t28100;
    let t97349 = -t97310 / F::cast_from(96.0_f64) + F::cast_from(0.33643963411783659045e-4_f64) * t97315 + t97318 / F::cast_from(1536.0_f64) + t97320 / F::cast_from(384.0_f64) + t97322 / F::cast_from(192.0_f64) - F::cast_from(0.20186378047070195427e-3_f64) * t97326 - F::cast_from(0.63250651214153279005e-2_f64) * t91206 - t91215 - F::cast_from(0.67826230238155856634e-1_f64) * t80767 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t80776 + t91226 - F::cast_from(0.12111826828242117256e-2_f64) * t97333 + F::cast_from(0.40372756094140390854e-3_f64) * t97337 - F::cast_from(0.31625325607076639503e-2_f64) * t80780 - t97340 / F::cast_from(384.0_f64) - t97342 / F::cast_from(192.0_f64) - t97344 / F::cast_from(192.0_f64) - F::cast_from(0.40372756094140390853e-3_f64) * t97347;
    t97349
}
