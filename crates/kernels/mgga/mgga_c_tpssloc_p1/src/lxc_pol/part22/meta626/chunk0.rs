//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2160/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2160<F: Float>(t16060: F, t3865: F, t1831: F, t40292: F, t12345: F, t5314: F, t40018: F, t5223: F, t12282: F, t5234: F, t12189: F, t5227: F) -> (F, F, F, F, F, F) {
    let t53906 = t16060 * t3865;
    let t53917 = t40292 * t1831;
    let t53918 = F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t53917;
    let t53919 = t12345 * t5314;
    let t53920 = F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t53919;
    let t53927 = t40018 * t5223;
    let t53928 = F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t53927;
    let t53945 = t5234 * t12282;
    let t53984 = t12189 * t5227;
    (t53906, t53918, t53920, t53928, t53945, t53984)
}
