//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 966/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk966<F: Float>(t1945: F, t4552: F, t1603: F, t6768: F, t23384: F, t7557: F, t4693: F, t6705: F, t6704: F, t14555: F, t1635: F, t1956: F, t23327: F, t23369: F, t23392: F, t23579: F, t25798: F, t25802: F, t25807: F, t25811: F, t25816: F, t3169: F, t388: F, t4557: F, t6680: F, t6687: F, t6816: F, t7562: F, t7625: F) -> (F,) {
    let t25820 = t4552 * t1945;
    let t25822 = t1603 * t6768;
    let t25824 = t23384 * t7557;
    let t25826 = t6705 * t4693;
    let t25827 = t6704 * t25826;
    let t25834 = 0.27415567780803773942e-2 * t23392 - 0.82246703342411321825e-2 * t6687 * t25798 + 0.27415567780803773942e-2 * t6687 * t25802 - t23369 * t1635 + 0.27415567780803773942e-2 * t25807 + 0.91385225936012579807e-3 * t23579 + 0.27415567780803773942e-2 * t6687 * t25811 - 0.27415567780803773942e-2 * t23327 * t25816 - t3169 * t7625 + t25820 * t388 + t25822 * t388 - 0.27415567780803773942e-2 * t25824 - 0.82246703342411321825e-2 * t6687 * t25827 - t4557 * t6816 - t14555 * t1956 - 0.21932454224643019153e-1 * t6680 * t7562;
    (t25834,)
}
