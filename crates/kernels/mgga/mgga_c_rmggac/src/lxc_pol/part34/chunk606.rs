//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 606/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk606<F: Float>(t15382: F, t515: F, t1971: F, t7230: F, t236: F, t598: F, t14125: F, t14124: F, t2304: F, t14131: F, t3352: F, t14225: F) -> (F, F, F, F, F, F, F, F) {
    let t15383 = t515 * t15382;
    let t15384 = t1971 * t15383;
    let t15385 = t7230 * t15384;
    let t15386 = F::new(0.1064114997332445985e-4) * t15385;
    let t15387 = t236 * t598;
    let t15388 = t14125 * t15387;
    let t15389 = t14124 * t15388;
    let t15391 = t14125 * t2304;
    let t15392 = t14131 * t15391;
    let t15394 = t3352 * t2304;
    let t15395 = t14225 * t15394;
    (t15384, t15386, t15388, t15389, t15391, t15392, t15394, t15395)
}
