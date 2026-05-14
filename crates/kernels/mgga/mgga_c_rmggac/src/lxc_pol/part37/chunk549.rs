//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 549/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk549<F: Float>(t15062: F, t15064: F, t15076: F, t15079: F, t2868: F, t3188: F, t3194: F, t5928: F, t209: F, t605: F, t698: F, t515: F, t1971: F, t1970: F, t15187: F, t15189: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15430 = 0.72042316457491791901e-3 * t15062;
    let t15431 = 0.38430329123504567781e-4 * t15064;
    let t15433 = 0.44903406381989282115e-1 * t15076;
    let t15434 = 0.14967802127329760705e-1 * t15079;
    let t15437 = t2868 * t3188;
    let t15438 = 0.14967802127329760705e-1 * t15437;
    let t15445 = t5928 * t3194;
    let t15446 = 0.39914139006212695214e-1 * t15445;
    let t15448 = t698 * t605 * t209;
    let t15449 = t515 * t15448;
    let t15450 = t1971 * t15449;
    let t15451 = t1970 * t15450;
    let t15452 = 0.42564599893297839398e-5 * t15451;
    let t15453 = 0.20455996240684006298e-1 * t15187;
    let t15454 = 0.2727466165424534173e-1 * t15189;
    (t15430, t15431, t15433, t15434, t15438, t15446, t15450, t15452, t15453, t15454)
}
