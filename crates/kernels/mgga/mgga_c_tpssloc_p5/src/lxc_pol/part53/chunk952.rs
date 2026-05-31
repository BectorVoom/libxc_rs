//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 952/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk952<F: Float>(t114034: F, t114046: F, t31560: F, t6914: F, t31590: F, t6883: F, t22724: F, t31594: F, t2085: F, t213: F, t225: F, t22642: F, t22643: F, t8621: F) -> (F, F, F, F, F, F, F) {
    let t115464 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t114034;
    let t115467 = F::cast_from(0.5383034145885385447e-3_f64) * t114046;
    let t115508 = t6914 * t31560;
    let t115530 = t6883 * t31590;
    let t115539 = t22724 * t31594;
    let t115545 = t213 * t2085 * t225;
    let t115550 = t22642 * t22643 * t8621;
    (t115464, t115467, t115508, t115530, t115539, t115545, t115550)
}
