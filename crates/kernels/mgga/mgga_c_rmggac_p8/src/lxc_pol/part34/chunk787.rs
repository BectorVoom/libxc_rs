//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 787/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk787<F: Float>(t13872: F, t74170: F, t13876: F, t13880: F, t13884: F, t13848: F, t8511: F, t13850: F, t14229: F, t39207: F, t14233: F, t3069: F, t40193: F) -> (F, F, F, F, F, F, F, F) {
    let t74171 = t74170 * t13872;
    let t74173 = t74170 * t13876;
    let t74175 = t74170 * t13880;
    let t74177 = t74170 * t13884;
    let t74179 = t8511 * t13848;
    let t74180 = t74179 * t13850;
    let t74182 = t39207 * t14229;
    let t74183 = t74182 * t14233;
    let t74191 = t40193 * t3069;
    (t74171, t74173, t74175, t74177, t74179, t74180, t74183, t74191)
}
