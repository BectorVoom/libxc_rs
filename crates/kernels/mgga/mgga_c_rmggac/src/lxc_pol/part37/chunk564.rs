//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 564/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk564<F: Float>(t15887: F, t82: F, t72: F, t15206: F, t15212: F, t15224: F, t15865: F, t884: F, t1356: F, t15872: F, t15273: F, t14953: F, t530: F, t15881: F, t515: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15888 = t82 * t15887;
    let t15889 = t72 * t15888;
    let t15892 = 0.87596530464506835932e-6 * t15206;
    let t15893 = 0.87596530464506835932e-6 * t15212;
    let t15894 = 0.17519306092901367187e-6 * t15224;
    let t15899 = t884 * t15865;
    let t15900 = 0.59871208509319042821e-1 * t15899;
    let t15901 = t1356 * t15872;
    let t15902 = 0.39914139006212695214e-1 * t15901;
    let t15903 = 0.31062809106223861414e-2 * t15273;
    let t15904 = t530 * t14953;
    let t15905 = 0.2363e1 * t15904;
    let t15907 = t515 * t15881;
    let t15908 = t235 * t15907;
    (t15888, t15889, t15892, t15893, t15894, t15900, t15902, t15903, t15905, t15907, t15908)
}
