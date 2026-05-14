//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 945/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk945<F: Float>(t25653: F, t25654: F, t1015: F, t1615: F, t1011: F, t1022: F, t360: F, t1941: F, t4616: F, t23474: F, t23480: F, t23483: F, t23500: F, t23564: F, t25639: F, t25642: F, t25645: F, t25652: F, t378: F, t4585: F, t4609: F, t6717: F, t6747: F, t6765: F, t7583: F) -> (F,) {
    let t25655 = t25653 * t25654;
    let t25658 = t1015 * t1615;
    let t25659 = t1011 * t1022;
    let t25660 = t25659 * t360;
    let t25661 = t25658 * t25660;
    let t25664 = t4616 * t1941;
    let t25672 = 0.10093189023535097714e-3 * t23474 - 0.10093189023535097714e-3 * t23480 + t6717 * t4609 / 288.0 - 0.10093189023535097714e-3 * t25639 + 0.10093189023535097714e-3 * t25642 - 0.10093189023535097714e-3 * t25645 * t6747 - 0.10093189023535097714e-3 * t23564 * t7583 + 0.20186378047070195428e-3 * t25652 * t25655 - 0.10093189023535097714e-3 * t25652 * t25661 + t25664 * t378 / 1536.0 + t23500 / 2304.0 - 0.80745512188280781712e-3 * t23483 * t7583 - t6765 * t4585 / 1152.0;
    (t25672,)
}
