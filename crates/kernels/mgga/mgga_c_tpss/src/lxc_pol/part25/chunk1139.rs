//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1139/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1139<F: Float>(t11850: F, t11875: F, t11932: F, t12024: F, t12035: F, t12046: F, t15237: F, t15239: F, t15241: F, t15243: F, t15245: F, t15259: F, t15321: F, t15324: F, t15327: F, t15330: F, t15669: F, t15690: F, t9192: F, t9221: F, t9429: F, t9438: F) -> F {
    let t15692 = -t9429 + F::new(0.11577222222222222222e0) * t9192 - t12024 + F::new(0.23154444444444444445e0) * t11850 - t9438 + F::new(0.104195e0) * t15237 + F::new(0.11477222222222222222e0) * t15239 - F::new(0.34431666666666666667e0) * t15241 - F::new(0.17215833333333333333e0) * t15243 - F::new(0.13892666666666666667e0) * t15245 + t15669 + t12035 - F::new(0.68863333333333333332e0) * t11875 - t12046 + F::new(0.22954444444444444444e0) * t9221 + F::new(0.4630888888888888889e-1) * t11932 + F::new(0.57386111111111111112e0) * t15259 + F::new(0.41678e0) * t15321 + F::new(0.62517e0) * t15324 + F::new(0.20839e0) * t15327 - F::new(0.69463333333333333334e-1) * t15330 + t15690;
    t15692
}
