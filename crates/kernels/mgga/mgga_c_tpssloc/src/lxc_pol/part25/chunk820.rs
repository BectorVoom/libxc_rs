//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 820/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk820<F: Float>(t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F, t423: F, t11177: F, t11365: F, t11366: F, t1138: F, t11400: F, t11405: F, t11409: F, t11410: F, t11415: F, t11420: F, t11421: F, t11426: F, t11429: F, t11430: F, t11434: F, t11437: F, t11441: F, t11455: F, t1148: F, t3327: F, t3332: F, t3352: F, t3357: F, t3360: F, t3376: F, t3401: F, t436: F) -> (F, F) {
    let t11459 = 0.55403703703703703703e-1 * t11135;
    let t11470 = -t11459 + 0.23744444444444444444e-1 * t11137 + 0.11872222222222222222e-1 * t11139 - 0.35616666666666666666e-1 * t11141 - 0.17808333333333333333e-1 * t11143 + 0.19787037037037037037e-1 * t11150 - 0.71233333333333333332e-1 * t11156 - 0.35616666666666666666e-1 * t11161 + 0.10685e0 * t11165 + 0.10685e0 * t11170 + 0.17808333333333333333e-1 * t11174;
    let t11472 = 0.621814e-1 * t11470 * t423;
    let t11473 = -0.10389515463408878255e3 * t11365 * t11366 + 0.5848223622634646207e0 * t1148 * t11400 + t11405 - t11409 + 3.0 * t11410 * t1138 + 3.0 * t3327 * t3352 + 0.96491876992155210402e2 * t11415 * t3360 - 0.19298375398431042081e3 * t11420 * t11421 + t11426 - t11429 - 0.35089341735807877242e1 * t3376 * t11430 + 0.51947577317044391277e2 * t3401 * t11434 - 6.0 * t3332 * t11437 + 0.96491876992155210402e2 * t3357 * t11441 - 0.310907e-1 * t11455 * t436 - 0.19751673498613801407e-1 * t11177 + t11472;
    (t11472, t11473)
}
