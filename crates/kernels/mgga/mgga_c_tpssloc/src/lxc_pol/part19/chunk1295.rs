//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1295/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295<F: Float>(t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F, t43942: F, t43936: F, t449: F, t300: F, t1098: F, t11470: F) -> (F, F, F) {
    let t43949 = -0.27469135802469135803e-1 * t43811 + 0.24722222222222222222e-1 * t43727 - 0.74166666666666666668e-1 * t43729 + 0.61805555555555555555e-1 * t43734 - 0.38456790123456790123e-1 * t43816 + t43942 - 0.22249999999999999999e0 * t43737 - 0.18541666666666666666e-1 * t43823 - 0.24722222222222222222e-1 * t43740 + 0.33375e0 * t43743 + 0.55625000000000000001e-1 * t43828 + 0.74166666666666666668e-1 * t43746;
    let t43951 = (t43936 + t43949) * t449;
    let t43953 = 0.19751673498613801407e-1 * t300 * t43951;
    let t43954 = t11470 * t1098;
    (t43951, t43953, t43954)
}
