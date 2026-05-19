//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 677/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk677<F: Float>(t14255: F, t68524: F, t14018: F, t7715: F, t3119: F, t3899: F, t464: F, t14024: F, t14122: F, t14127: F, t14130: F, t68489: F) -> (F, F, F, F, F, F, F) {
    let t68525 = t68524 * t14255;
    let t68526 = F::cast_from(0.29085809927086856922e-4_f64) * t68525;
    let t68527 = t14018 * t7715;
    let t68528 = t68527 * t3119;
    let t68536 = t464 * t3899;
    let t68537 = t68536 * t14024;
    let t68538 = t14122 * t68537;
    let t68539 = t68538 * t14127;
    let t68540 = F::cast_from(0.16351352353374609375e-5_f64) * t68539;
    let t68541 = t14130 * t68489;
    (t68526, t68527, t68528, t68536, t68538, t68540, t68541)
}
