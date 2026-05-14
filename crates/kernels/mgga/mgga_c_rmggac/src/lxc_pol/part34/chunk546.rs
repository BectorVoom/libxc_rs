//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 546/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk546<F: Float>(t1356: F, t15421: F, t15030: F, t15033: F, t15037: F, t15041: F, t15044: F, t15047: F, t15062: F, t15064: F, t15068: F, t15076: F, t15079: F, t15082: F, t2868: F, t3188: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15422 = t1356 * t15421;
    let t15423 = 0.39914139006212695214e-1 * t15422;
    let t15424 = 0.3252672799280962148e-5 * t15030;
    let t15425 = 0.3252672799280962148e-5 * t15033;
    let t15426 = 0.30487649791575028312e-3 * t15037;
    let t15427 = 0.30487649791575028312e-3 * t15041;
    let t15428 = 0.16263363996404810741e-4 * t15044;
    let t15429 = 0.16263363996404810741e-4 * t15047;
    let t15430 = 0.72042316457491791901e-3 * t15062;
    let t15431 = 0.38430329123504567781e-4 * t15064;
    let t15432 = 0.17519306092901367187e-5 * t15068;
    let t15433 = 0.44903406381989282115e-1 * t15076;
    let t15434 = 0.14967802127329760705e-1 * t15079;
    let t15435 = 0.76860658247009135557e-5 * t15082;
    let t15437 = t2868 * t3188;
    (t15423, t15424, t15425, t15426, t15427, t15428, t15429, t15430, t15431, t15432, t15433, t15434, t15435, t15437)
}
