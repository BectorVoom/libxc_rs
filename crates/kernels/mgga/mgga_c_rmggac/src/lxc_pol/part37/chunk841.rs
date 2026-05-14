//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 841/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk841<F: Float>(t77939: F, t72023: F, t8650: F, t71951: F, t352: F, t5148: F, t77901: F, t71960: F, t76236: F, t14509: F, t8672: F, t14512: F, t8533: F, t2447: F, t664: F, t321: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77940 = 0.40911992481368012592e-1 * t77939;
    let t77941 = t72023 * t8650;
    let t77942 = 0.20455996240684006296e-1 * t77941;
    let t77943 = 0.79828278012425390426e-1 * t71951;
    let t77945 = t5148 * t77901 * t352;
    let t77946 = 0.2993560425465952141e-1 * t77945;
    let t77949 = 0.79828278012425390426e-1 * t71960;
    let t77950 = 0.18183107769496894487e-1 * t76236;
    let t77954 = t14509 * t8672;
    let t77955 = 0.36366215538993788971e-1 * t77954;
    let t77956 = t14512 * t8533;
    let t77957 = 0.18183107769496894486e-1 * t77956;
    let t77960 = t2447 * t664;
    let t77963 = 0.11974241701863808564e0 * t5148 * t77960 * t321;
    (t77940, t77942, t77943, t77946, t77949, t77950, t77955, t77957, t77960, t77963)
}
