//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 608/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk608<F: Float>(t15037: F, t15041: F, t15044: F, t15047: F, t15062: F, t15064: F, t15076: F, t15079: F, t2868: F, t3188: F, t3194: F, t5928: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15426 = F::new(0.30487649791575028312e-3) * t15037;
    let t15427 = F::new(0.30487649791575028312e-3) * t15041;
    let t15428 = F::new(0.16263363996404810741e-4) * t15044;
    let t15429 = F::new(0.16263363996404810741e-4) * t15047;
    let t15430 = F::new(0.72042316457491791901e-3) * t15062;
    let t15431 = F::new(0.38430329123504567781e-4) * t15064;
    let t15433 = F::new(0.44903406381989282115e-1) * t15076;
    let t15434 = F::new(0.14967802127329760705e-1) * t15079;
    let t15437 = t2868 * t3188;
    let t15438 = F::new(0.14967802127329760705e-1) * t15437;
    let t15445 = t5928 * t3194;
    (t15426, t15427, t15428, t15429, t15430, t15431, t15433, t15434, t15438, t15445)
}
