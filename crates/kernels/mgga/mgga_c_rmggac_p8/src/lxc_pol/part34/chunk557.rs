//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 557/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk557<F: Float>(t14494: F, t338: F, t118: F, t2123: F, t2211: F, t3204: F, t321: F, t2085: F, t3191: F, t2228: F, t326: F, t650: F) -> (F, F, F, F, F, F, F, F) {
    let t14495 = t338 * t14494;
    let t14496 = t118 * t14495;
    let t14498 = t2211 * t2123;
    let t14500 = F::cast_from(0.39914139006212695214e-1_f64) * t118 * t14498;
    let t14501 = t3204 * t321;
    let t14504 = t3191 * t2085;
    let t14505 = F::cast_from(0.90915538847484472429e-2_f64) * t14504;
    let t14506 = t326 * t2228;
    let t14507 = t14506 * t650;
    (t14495, t14496, t14498, t14500, t14501, t14505, t14506, t14507)
}
