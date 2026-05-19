//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 610/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk610<F: Float>(t15064: F, t15068: F, t15076: F, t15079: F, t15082: F, t2868: F, t3188: F, t3204: F, t551: F, t739: F, t558: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15431 = F::cast_from(0.38430329123504567781e-4_f64) * t15064;
    let t15432 = F::cast_from(0.17519306092901367187e-5_f64) * t15068;
    let t15433 = F::cast_from(0.44903406381989282115e-1_f64) * t15076;
    let t15434 = F::cast_from(0.14967802127329760705e-1_f64) * t15079;
    let t15435 = F::cast_from(0.76860658247009135557e-5_f64) * t15082;
    let t15437 = t2868 * t3188;
    let t15438 = F::cast_from(0.14967802127329760705e-1_f64) * t15437;
    let t15439 = t3204 * t551;
    let t15440 = t739 * t15439;
    let t15441 = F::cast_from(0.59871208509319042821e-1_f64) * t15440;
    let t15442 = t3204 * t558;
    let t15443 = t884 * t15442;
    (t15431, t15432, t15433, t15434, t15435, t15438, t15439, t15441, t15442, t15443)
}
