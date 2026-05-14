//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 951/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk951<F: Float>(t11584: F, t2740: F, t3944: F, t8983: F, t2459: F, t969: F, t1460: F, t672: F, t925: F, t140: F, t3927: F, t2682: F, t3941: F, t8493: F, t8539: F, t1465: F, t242: F, t8469: F) -> (F, F, F, F, F, F, F, F) {
    let t11586 = t2740 * t11584 / 3456.0;
    let t11588 = t8983 * t3944;
    let t11590 = t2740 * t11588 / 3456.0;
    let t11621 = t969 * t2459;
    let t11640 = t672 * t1460;
    let t11641 = t925 * t11640;
    let t11645 = t140 * t3927;
    let t11647 = t925 * t11645 / 432.0;
    let t11659 = t2682 * t3941 / 432.0;
    let t11661 = t8539 * t8493;
    let t11687 = t242 * t8469 * t1465;
    (t11586, t11590, t11621, t11641, t11647, t11659, t11661, t11687)
}
