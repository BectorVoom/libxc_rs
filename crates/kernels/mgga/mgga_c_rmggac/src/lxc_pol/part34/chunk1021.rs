//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1021/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1021<F: Float>(t77744: F, t15624: F, t498: F, t515: F, t7230: F, t7231: F, t321: F, t3352: F, t1971: F, t2144: F, t333: F, t352: F, t875: F) -> (F, F, F, F, F) {
    let t77745 = F::new(0.25538759935978703638e-4) * t77744;
    let t77749 = t7230 * t7231 * t515 * t15624 * t498;
    let t77750 = F::new(0.53205749866622299248e-5) * t77749;
    let t77754 = t7230 * t3352 * t515 * t15624 * t321;
    let t77755 = F::new(0.15961724959986689774e-4) * t77754;
    let t77759 = t7230 * t1971 * t2144 * t15624 * t333;
    let t77760 = F::new(0.15961724959986689774e-4) * t77759;
    let t77764 = t7230 * t1971 * t875 * t15624 * t352;
    (t77745, t77750, t77755, t77760, t77764)
}
