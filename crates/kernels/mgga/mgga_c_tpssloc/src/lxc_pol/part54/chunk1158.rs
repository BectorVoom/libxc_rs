//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1158/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1158<F: Float>(t31632: F, t6883: F, t22724: F, t31623: F, t22716: F, t8631: F, t31631: F, t6897: F, t794: F, t113981: F, t114025: F, t114027: F, t114038: F, t1338: F, t31584: F, t31560: F, t6914: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    let t115433 = 0.26044789391763585244e-1 * t115432;
    let t115434 = t22716 * t8631;
    let t115435 = 0.63969658155208805863e-1 * t115434;
    let t115439 = t6897 * t794 * t31631;
    let t115447 = 0.13457585364713463618e-3 * t113981;
    let t115461 = 0.42167100809435519335e-2 * t114025;
    let t115462 = 0.90434973650874475512e-1 * t114027;
    let t115465 = 119.0 / 3456.0 * t114038;
    let t115486 = t1338 * t31584;
    let t115508 = t6914 * t31560;
    (t115430, t115433, t115435, t115439, t115447, t115461, t115462, t115465, t115486, t115508)
}
