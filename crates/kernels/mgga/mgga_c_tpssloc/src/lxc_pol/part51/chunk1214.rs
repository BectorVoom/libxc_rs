//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1214/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1214<F: Float>(t4021: F, t8513: F, t8514: F, t1862: F, t641: F, t1433: F, t31691: F, t4017: F, t45844: F, t8511: F, t115903: F, t119901: F, t119891: F, t115833: F, t119883: F, t119879: F) -> (F, F, F, F, F, F, F, F) {
    let t121074 = t8513 * t8514 * t4021;
    let t121079 = t641 * t1862;
    let t121081 = t8513 * t121079 * t1433;
    let t121087 = t8513 * t31691 * t4017;
    let t121094 = t45844 * t8511;
    let t121099 = t115903 * t119901;
    let t121102 = t115903 * t119891;
    let t121105 = t115833 * t119883;
    let t121108 = t115833 * t119879;
    (t121074, t121081, t121087, t121094, t121099, t121102, t121105, t121108)
}
