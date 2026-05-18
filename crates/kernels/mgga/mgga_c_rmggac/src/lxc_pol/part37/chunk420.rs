//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 420/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk420<F: Float>(t262: F, t8645: F, t2347: F, t352: F, t1679: F, t511: F, t498: F, t615: F, t236: F, t2084: F, t558: F, t27: F) -> (F, F, F, F, F, F) {
    let t8646 = t262 * t8645;
    let t8649 = t2347 * t352;
    let t8650 = t262 * t8649;
    let t8659 = t1679 * t511;
    let t8666 = t615 * t498;
    let t8667 = t236 * t8666;
    let t8671 = t2084 * t558;
    let t8672 = t27 * t8671;
    (t8646, t8649, t8650, t8659, t8667, t8672)
}
