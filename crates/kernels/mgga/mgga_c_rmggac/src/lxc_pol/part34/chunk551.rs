//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 551/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk551<F: Float>(t15502: F, t515: F, t7231: F, t3351: F, t15255: F, t15259: F, t15263: F, t15267: F, t15269: F, t15273: F, t664: F, t9530: F, t1356: F, t3230: F, t623: F, t2412: F, t3219: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15503 = t515 * t15502;
    let t15504 = t7231 * t15503;
    let t15505 = t3351 * t15504;
    let t15506 = 0.42564599893297839398e-5 * t15505;
    let t15510 = 0.85129199786595678799e-5 * t15255;
    let t15511 = 0.2553875993597870364e-4 * t15259;
    let t15512 = 0.2553875993597870364e-4 * t15263;
    let t15513 = 0.1702583995731913576e-4 * t15267;
    let t15514 = 0.85129199786595678799e-5 * t15269;
    let t15515 = 0.31062809106223861415e-2 * t15273;
    let t15516 = t9530 * t664;
    let t15517 = t1356 * t15516;
    let t15518 = 0.39914139006212695214e-1 * t15517;
    let t15519 = t623 * t3230;
    let t15520 = 0.19957069503106347607e-1 * t15519;
    let t15521 = t2412 * t3219;
    (t15504, t15506, t15510, t15511, t15512, t15513, t15514, t15515, t15516, t15518, t15520, t15521)
}
