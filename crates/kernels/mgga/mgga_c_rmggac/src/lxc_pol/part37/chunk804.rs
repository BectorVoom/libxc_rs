//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 804/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk804<F: Float>(t71167: F, t77044: F, t637: F, t8905: F, t71007: F, t8621: F, t72138: F, t74498: F, t74501: F, t74503: F, t15523: F, t2191: F, t1986: F, t675: F, t9566: F, t68660: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77045 = t71167 * t77044;
    let t77046 = 0.20455996240684006297e-1 * t77045;
    let t77047 = t637 * t8905;
    let t77048 = t71007 * t77047;
    let t77049 = 0.27274661654245341729e-1 * t77048;
    let t77050 = t637 * t8621;
    let t77051 = t72138 * t77050;
    let t77052 = 0.20455996240684006297e-1 * t77051;
    let t77054 = 0.1276937996798935182e-4 * t74498;
    let t77055 = 0.3830813990396805546e-4 * t74501;
    let t77056 = 0.1276937996798935182e-4 * t74503;
    let t77057 = t2191 * t15523;
    let t77058 = 0.42564599893297839398e-5 * t77057;
    let t77060 = t675 * t1986 * t9566;
    let t77061 = 0.42564599893297839398e-5 * t77060;
    let t77062 = 0.638468998399467591e-4 * t68660;
    (t77046, t77049, t77052, t77054, t77055, t77056, t77058, t77061, t77062)
}
