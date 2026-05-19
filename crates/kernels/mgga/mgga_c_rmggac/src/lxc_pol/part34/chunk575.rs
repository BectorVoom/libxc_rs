//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 575/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk575<F: Float>(t14664: F, t498: F, t698: F, t515: F, t7231: F, t3351: F, t8235: F, t3352: F, t14211: F, t14214: F, t14217: F, t14220: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14665 = F::cast_from(0.42564599893297839398e-5_f64) * t14664;
    let t14666 = t698 * t498;
    let t14667 = t515 * t14666;
    let t14668 = t7231 * t14667;
    let t14669 = t3351 * t14668;
    let t14670 = F::cast_from(0.42564599893297839398e-5_f64) * t14669;
    let t14671 = t515 * t8235;
    let t14672 = t3352 * t14671;
    let t14673 = t3351 * t14672;
    let t14674 = F::cast_from(0.12769379967989351819e-4_f64) * t14673;
    let t14676 = F::cast_from(0.16263363996404810741e-4_f64) * t14211;
    let t14677 = F::cast_from(0.16263363996404810741e-4_f64) * t14214;
    let t14678 = F::cast_from(0.3252672799280962148e-5_f64) * t14217;
    let t14679 = F::cast_from(0.3252672799280962148e-5_f64) * t14220;
    (t14665, t14668, t14670, t14672, t14674, t14676, t14677, t14678, t14679)
}
