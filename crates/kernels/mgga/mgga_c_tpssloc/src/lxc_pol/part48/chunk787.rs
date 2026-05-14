//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 787/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk787<F: Float>(t5: F, t625: F, t8307: F, t8513: F, t8663: F, t31857: F, t31860: F, t31864: F, t31868: F, t32328: F, t32333: F, t32340: F, t8825: F, t112: F, t111: F, t8828: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t32343 = t8307 * t625;
    let t32344 = t8513 * t32343;
    let t32346 = 5.0 / 27.0 * t8663 * t32344;
    let t32348 = piecewise3(t8, 0.0, -5.0 / 72.0 * t31857 * t8825 + 5.0 / 12.0 * t31860 * t32328 + 5.0 / 18.0 * t31864 * t32333 - 5.0 / 72.0 * t31868 * t8825 - 5.0 / 36.0 * t8663 * t32340 + t32346);
    let t32349 = t32348 * t112;
    let t32350 = t8828 * t111;
    (t32343, t32344, t32348, t32349, t32350)
}
