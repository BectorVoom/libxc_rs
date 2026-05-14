//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1110/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1110<F: Float>(t2165: F, t7467: F, t652: F, t4028: F, t8675: F, t7458: F, t1873: F, t8103: F, t1458: F, t8682: F, t1849: F, t33084: F, t33086: F, t33088: F, t33092: F, t33725: F, t7266: F, t7472: F, t8329: F, t8687: F) -> (F, F, F, F) {
    let t33726 = t2165 * t7467;
    let t33727 = t652 * t33726;
    let t33731 = t4028 * t8675;
    let t33733 = t7458 * t8675;
    let t33735 = t8103 * t1873;
    let t33736 = t652 * t33735;
    let t33740 = t8682 * t1458;
    let t33743 = t1849 * t8687 - 2.0 * t33740 * t652 - 2.0 * t7266 * t7472 + t33084 - 2.0 * t33086 - 2.0 * t33088 - t33092 - t33725 - 2.0 * t33727 - 2.0 * t33731 - 2.0 * t33733 - 2.0 * t33736 - t8329;
    (t33726, t33735, t33740, t33743)
}
