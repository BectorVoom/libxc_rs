//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 573/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk573<F: Float>(t14970: F, t14119: F, t14128: F, t14133: F, t3282: F, t333: F, t884: F, t3281: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t14971 = F::new(0.59871208509319042821e-1) * t14970;
    let t14973 = F::new(0.17519306092901367186e-5) * t14119;
    let t14974 = F::new(0.35038612185802734374e-6) * t14128;
    let t14975 = F::new(0.35038612185802734374e-6) * t14133;
    let t14977 = t3282 * t333;
    let t14978 = t884 * t14977;
    let t14979 = F::new(0.59871208509319042821e-1) * t14978;
    let t14980 = t874 * t3281;
    (t14971, t14973, t14974, t14975, t14977, t14979, t14980)
}
