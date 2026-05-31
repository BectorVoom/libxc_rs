//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 733/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk733<F: Float>(t1364: F, t3548: F, t198: F, t2115: F, t2208: F, t2217: F, t2292: F, t2302: F, t2310: F, t2333: F, t2347: F, t2351: F, t2439: F, t4706: F, t4727: F, t4743: F, t4746: F) -> F {
    let t4814 = t3548 * t1364;
    let t4817 = F::cast_from(6.0_f64) * t198 * t2115 * t4706 + F::cast_from(6.0_f64) * t2439 * t4814 - t2208 - t2217 - t2292 + t2302 + t2310 + t2333 + t2347 + t2351 + t4727 + t4743 + t4746;
    t4817
}
