//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1216/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1216<F: Float>(t2717: F, t7841: F, t1888: F, t23270: F, t865: F, t25038: F, t31337: F, t4255: F, t112676: F, t114613: F, t114615: F, t118500: F, t118503: F, t118506: F, t118518: F, t118523: F, t2597: F, t2713: F, t2718: F, t33443: F, t7106: F, t7537: F, t855: F) -> (F,) {
    let t121349 = t2717 * t7841;
    let t121352 = t1888 * t23270 * t121349 * t865;
    let t121362 = t25038 * t23270 * t31337 * t4255;
    let t121364 = 2.0 * t2597 * t33443 + 2.0 * t2713 * t33443 + 0.16449340668482264365e-1 * t121352 - 0.82246703342411321824e-2 * t114613 + t118500 - t112676 + 2.0 * t855 * t2718 * t7106 * t7537 - 0.19190897446562641759e-1 * t114615 - t118503 - 0.49348022005446793095e-1 * t121362 + t118506 - t118518 - t118523;
    (t121364,)
}
