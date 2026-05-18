//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1246/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1246<F: Float>(t18397: F, t4645: F, t4669: F, t5527: F, t4674: F, t93: F, t94: F, t196: F, t197: F, t5322: F, t30: F, t4706: F) -> (F, F, F, F, F, F) {
    let t21185 = t18397 * t4645;
    let t21187 = t5527 * t4669;
    let t21227 = t93 * t4674;
    let t21236 = t94 * t4674;
    let t21253 = t5322 * t196 * t197;
    let t21255 = t30 * t4706;
    (t21185, t21187, t21227, t21236, t21253, t21255)
}
