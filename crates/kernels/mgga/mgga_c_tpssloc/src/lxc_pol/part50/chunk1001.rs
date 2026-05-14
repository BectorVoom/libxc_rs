//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1001/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1001<F: Float>(t30788: F, t7553: F, t6705: F, t7624: F, t6704: F, t30854: F, t7565: F, t1599: F, t8376: F, t1603: F, t8391: F, t32961: F, t349: F, t1635: F, t30912: F, t30915: F, t388: F, t4557: F, t4660: F, t6687: F, t6771: F, t7600: F, t7625: F, t8397: F, t8407: F) -> (F, F, F, F, F, F, F, F) {
    let t32987 = t30788 * t7553;
    let t32992 = t6705 * t7624;
    let t32993 = t6704 * t32992;
    let t32998 = t30854 * t7565;
    let t33001 = t1599 * t8376;
    let t33005 = t1603 * t8391;
    let t33007 = t349 * t32961;
    let t33012 = 2.0 * t4557 * t8397 + 0.54831135561607547883e-2 * t6687 * t32987 + 4.0 * t6771 * t7600 - 0.16449340668482264365e-1 * t6687 * t32993 + 2.0 * t4660 * t8397 - 0.16449340668482264365e-1 * t6687 * t32998 - 0.16449340668482264365e-1 * t6687 * t33001 - t4660 * t8407 + t33005 * t388 + t33007 * t388 - t30915 * t1635 - 2.0 * t6771 * t7625 - t30912;
    (t32987, t32992, t32993, t32998, t33001, t33005, t33007, t33012)
}
