//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 547/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk547<F: Float>(t15437: F, t3204: F, t551: F, t739: F, t558: F, t884: F, t3194: F, t5928: F, t209: F, t605: F, t698: F, t515: F, t1971: F, t1970: F, t15187: F, t15189: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15438 = 0.14967802127329760705e-1 * t15437;
    let t15439 = t3204 * t551;
    let t15440 = t739 * t15439;
    let t15441 = 0.59871208509319042821e-1 * t15440;
    let t15442 = t3204 * t558;
    let t15443 = t884 * t15442;
    let t15444 = 0.59871208509319042821e-1 * t15443;
    let t15445 = t5928 * t3194;
    let t15446 = 0.39914139006212695214e-1 * t15445;
    let t15448 = t698 * t605 * t209;
    let t15449 = t515 * t15448;
    let t15450 = t1971 * t15449;
    let t15451 = t1970 * t15450;
    let t15452 = 0.42564599893297839398e-5 * t15451;
    let t15453 = 0.20455996240684006298e-1 * t15187;
    let t15454 = 0.2727466165424534173e-1 * t15189;
    (t15438, t15439, t15441, t15442, t15444, t15446, t15450, t15452, t15453, t15454)
}
