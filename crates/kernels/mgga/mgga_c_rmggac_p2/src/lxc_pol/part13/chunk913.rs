//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 913/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk913<F: Float>(t40138: F, t7284: F, t34975: F, t34976: F, t571: F, t7455: F, t39850: F, t7229: F, t4550: F, t495: F, t8440: F, t35039: F, t39851: F, t498: F) -> (F, F, F, F) {
    let t40139 = t40138 * t7284;
    let t40143 = t34975 * t34976 * t571 * t7455;
    let t40145 = t7229 * t39850;
    let t40149 = t40145 * t34976 * t8440 * t4550 * t495;
    let t40154 = t39851 * t35039 * t8440 * t4550 * t498;
    (t40139, t40143, t40149, t40154)
}
