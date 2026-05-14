//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1103/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1103<F: Float>(t5: F, t119941: F, t119993: F, t112: F, t32781: F, t532: F, t1983: F, t6879: F, t26149: F, t8450: F, t33133: F, t7000: F, t33160: F, t6876: F, t119867: F, t119869: F, t119871: F, t119874: F, t119875: F, t119877: F, t1266: F, t1442: F, t22461: F, t26103: F, t30989: F, t32679: F, t33124: F, t4026: F, t510: F, t7472: F, t8329: F, t8439: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t119995 = piecewise3(t8, 0.0, t119941 + t119993);
    let t119996 = t119995 * t112;
    let t119999 = t532 * t32781;
    let t120002 = 3.0 * t1983 * t119999 * t6879;
    let t120003 = t8450 * t26149;
    let t120005 = t33133 * t7000;
    let t120008 = 3.0 * t6876 * t33160;
    let t120015 = -t119996 * t510 - t1266 * t33124 - t1442 * t30989 - 4.0 * t22461 * t7472 - 4.0 * t26103 * t7472 - t4026 * t8439 - t119867 - 4.0 * t119869 - 4.0 * t119871 - t119874 + 2.0 * t119875 + t119877 + t120002 - 2.0 * t120003 - 2.0 * t120005 - t120008 - t32679 - t8329;
    (t119996, t120015)
}
