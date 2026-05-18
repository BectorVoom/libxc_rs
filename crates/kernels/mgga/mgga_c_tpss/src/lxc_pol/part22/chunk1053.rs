//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1053/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1053<F: Float>(t11453: F, t3955: F, t2731: F, t3978: F, t967: F, t3973: F, t10416: F, t3972: F, t3931: F, t10412: F, t2761: F, t8444: F) -> (F, F, F, F, F, F) {
    let t11454 = t11453 * t3955;
    let t11456 = t2731 * t11454 / F::new(2304.0);
    let t11457 = t11453 * t3978;
    let t11459 = t967 * t11457 / F::new(1728.0);
    let t11460 = t11453 * t3973;
    let t11462 = F::new(5.0) / F::new(10368.0) * t967 * t11460;
    let t11463 = t3972 * t10416;
    let t11464 = t3931 * t11463;
    let t11467 = t3972 * t10412;
    let t11468 = t3931 * t11467;
    let t11475 = t2761 * t8444;
    (t11456, t11459, t11462, t11464, t11468, t11475)
}
