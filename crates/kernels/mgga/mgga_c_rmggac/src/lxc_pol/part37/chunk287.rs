//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 287/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk287<F: Float>(t118: F, t2471: F, t2200: F, t2204: F, t2382: F, t2384: F, t2386: F, t2388: F, t2390: F, t2394: F, t2464: F, t2467: F, t2469: F, t82: F, t534: F, t702: F) -> (F, F, F, F) {
    let t2472 = t118 * t2471;
    let t2474 = 0.5987120850931904282e-1 * t2382 - 0.8980681276397856423e-1 * t2384 - 0.2993560425465952141e-1 * t2386 - t2200 - 0.20455996240684006298e-1 * t2388 + 0.2727466165424534173e-1 * t2390 + 0.68186654135613354325e-2 * t2394 + t2204 + 0.59871208509319042821e-1 * t2464 - 0.59871208509319042821e-1 * t2467 - 0.39914139006212695214e-1 * t2469 + 0.19957069503106347607e-1 * t2472;
    let t2475 = t82 * t2474;
    let t2479 = t534 * t702;
    (t2472, t2474, t2475, t2479)
}
