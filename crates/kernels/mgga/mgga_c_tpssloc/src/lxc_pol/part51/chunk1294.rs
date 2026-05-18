//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1294/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1294<F: Float>(t114038: F, t1338: F, t31584: F, t31560: F, t6914: F, t225: F, t31573: F, t31590: F, t6883: F, t22724: F, t31594: F, t2085: F, t213: F) -> (F, F, F, F, F, F, F) {
    let t115465 = F::new(119.0) / F::new(3456.0) * t114038;
    let t115486 = t1338 * t31584;
    let t115508 = t6914 * t31560;
    let t115519 = t31573 * t225;
    let t115530 = t6883 * t31590;
    let t115539 = t22724 * t31594;
    let t115540 = F::new(0.26044789391763585244e-1) * t115539;
    let t115545 = t213 * t2085 * t225;
    (t115465, t115486, t115508, t115519, t115530, t115540, t115545)
}
