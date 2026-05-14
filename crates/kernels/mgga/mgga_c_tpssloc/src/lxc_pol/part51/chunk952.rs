//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 952/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk952<F: Float>(t343: F, t381: F, t6690: F, t25712: F, t4347: F, t6689: F, t7561: F, t968: F, t1920: F, t1625: F, t6688: F, t6691: F, t1065: F, t1409: F, t23330: F, t23329: F) -> (F, F, F, F, F) {
    let t25796 = t343 * t381;
    let t25797 = t25796 * t6690;
    let t25798 = t25712 * t25797;
    let t25801 = t6690 * t4347;
    let t25802 = t6689 * t25801;
    let t25806 = t968 * t7561;
    let t25807 = t1920 * t25806;
    let t25810 = t6688 * t1625;
    let t25811 = t25810 * t6691;
    let t25814 = t1409 * t1065;
    let t25815 = t23330 * t25814;
    let t25816 = t23329 * t25815;
    (t25798, t25802, t25807, t25811, t25816)
}
