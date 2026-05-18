//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1036/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1036<F: Float>(t352: F, t5148: F, t77901: F, t71960: F, t76236: F, t14509: F, t8672: F, t14512: F, t8533: F, t333: F, t4669: F, t76228: F, t76232: F, t77894: F, t77933: F, t77935: F, t77938: F, t77940: F, t77942: F, t77943: F) -> F {
    let t77945 = t5148 * t77901 * t352;
    let t77946 = F::new(0.2993560425465952141e-1) * t77945;
    let t77949 = F::new(0.79828278012425390426e-1) * t71960;
    let t77950 = F::new(0.18183107769496894487e-1) * t76236;
    let t77954 = t14509 * t8672;
    let t77955 = F::new(0.36366215538993788971e-1) * t77954;
    let t77956 = t14512 * t8533;
    let t77957 = F::new(0.18183107769496894486e-1) * t77956;
    let t77958 = t77933 - t77935 + t77938 - t77940 - t77942 + t77943 + t77946 - F::new(0.82834157616596963776e-1) * t76228 - F::new(0.16566831523319392755e-1) * t76232 - t77949 - t77950 - F::new(0.17961362552795712846e0) * t4669 * t77894 * t333 + t77955 + t77957;
    t77958
}
