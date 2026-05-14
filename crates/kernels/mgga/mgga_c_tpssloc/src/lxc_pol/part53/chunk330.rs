//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 330/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk330<F: Float>(t1814: F, t554: F, t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1789: F, t1791: F, t225: F, t680: F, t705: F, t1347: F, t1799: F, t546: F, t548: F) -> (F, F, F, F) {
    let t1815 = t1814 * t554;
    let t1819 = (t680 + t705 - t1274 - t1276 + t1789 + t1288 + t1791 - t1293 - t1296) * t225;
    let t1821 = t1347 * t1799;
    let t1824 = -t1819 * t548 + 3.0 * t1821 * t546;
    (t1815, t1819, t1821, t1824)
}
