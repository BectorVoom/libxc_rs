//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 589/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk589<F: Float>(t2130: F, t1932: F, t2133: F, t2132: F, t7573: F, t1714: F, t460: F, t7320: F, t1734: F, t68: F, t475: F, t7328: F, t1730: F, t2140: F, t1742: F, t2139: F, rho1: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8025 = t2130 * rho1;
    let t8026 = 1.0 / t8025;
    let t8027 = t8026 * t1932;
    let t8028 = t8027 * t2133;
    let t8031 = t2132 * t7573;
    let t8034 = t1714 * t460;
    let t8035 = t8034 * t7320;
    let t8038 = t1734 * t68;
    let t8039 = t8038 * t475;
    let t8040 = t7328 * t8039;
    let t8043 = t1730 * t2140;
    let t8048 = t2139 * t1742;
    (t8026, t8027, t8028, t8031, t8034, t8035, t8039, t8040, t8043, t8048)
}
