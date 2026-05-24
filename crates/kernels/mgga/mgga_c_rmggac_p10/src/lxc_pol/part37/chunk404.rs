//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 404/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk404<F: Float>(t702: F, t934: F, t7582: F, t7594: F, t7627: F, t7662: F, t2231: F, t290: F, t333: F, t698: F, t321: F, t7818: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8048 = t934 * t702;
    let t8125 = F::cast_from(0.29568125932752208315e-3_f64) * t7582;
    let t8129 = F::cast_from(0.22223798384940648817e-1_f64) * t7594;
    let t8143 = F::cast_from(0.97567895348519921633e-1_f64) * t7627;
    let t8156 = F::cast_from(0.12981128458281457309e-2_f64) * t7662;
    let t8188 = t290 * t2231;
    let t8231 = t698 * t333;
    let t8235 = t698 * t321;
    let t8242 = F::cast_from(0.2927036860455597649e0_f64) * t7818;
    (t8048, t8125, t8129, t8143, t8156, t8188, t8231, t8235, t8242)
}
