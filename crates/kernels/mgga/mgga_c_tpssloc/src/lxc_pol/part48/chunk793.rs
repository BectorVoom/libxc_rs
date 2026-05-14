//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 793/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk793<F: Float>(t32368: F, t32371: F, t32378: F, t32390: F, t3: F, t112: F, t8843: F, t2039: F, t24969: F, t24972: F, t31284: F, t31287: F, t31799: F, t31801: F, t31803: F, t31811: F, t31813: F, t31816: F, t31819: F, t577: F, t671: F, t7056: F, t7235: F, t7423: F, t8508: F) -> (F, F, F, F) {
    let t32392 = t32368 + t32371 + t32378 + t32390;
    let t32393 = t3 * t32392;
    let t32406 = t8843 * t112;
    let t32415 = 0.45e1 * t32392 * t577 + 0.135e2 * t32406 * t671 + 0.135e2 * t24969 * t2039 + 27.0 * t24972 * t7235 + 0.135e2 * t7423 * t7056 + t31799 + t31801 + t31803 + t31811 + t31813 + t31816 + t31819 + t31284 + t31287 + t8508;
    (t32392, t32393, t32406, t32415)
}
