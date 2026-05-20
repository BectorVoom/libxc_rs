//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1826/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1826<F: Float>(t25650: F, t25651: F, t1615: F, t3128: F, t1022: F, t23678: F, t1015: F, t1011: F, t360: F, t1941: F, t4616: F, t23474: F, t23480: F, t23483: F, t23500: F, t23564: F, t25639: F, t25642: F, t25645: F, t378: F, t4585: F, t4609: F, t6717: F, t6747: F, t6765: F, t7583: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25652 = t25650 * t25651;
    let t25653 = t3128 * t1615;
    let t25654 = t23678 * t1022;
    let t25655 = t25653 * t25654;
    let t25658 = t1015 * t1615;
    let t25659 = t1011 * t1022;
    let t25660 = t25659 * t360;
    let t25661 = t25658 * t25660;
    let t25664 = t4616 * t1941;
    let t25672 = F::cast_from(0.10093189023535097714e-3_f64) * t23474 - F::cast_from(0.10093189023535097714e-3_f64) * t23480 + t6717 * t4609 / F::new(288.0) - F::cast_from(0.10093189023535097714e-3_f64) * t25639 + F::cast_from(0.10093189023535097714e-3_f64) * t25642 - F::cast_from(0.10093189023535097714e-3_f64) * t25645 * t6747 - F::cast_from(0.10093189023535097714e-3_f64) * t23564 * t7583 + F::cast_from(0.20186378047070195428e-3_f64) * t25652 * t25655 - F::cast_from(0.10093189023535097714e-3_f64) * t25652 * t25661 + t25664 * t378 / F::new(1536.0) + t23500 / F::new(2304.0) - F::cast_from(0.80745512188280781712e-3_f64) * t23483 * t7583 - t6765 * t4585 / F::new(1152.0);
    (t25652, t25653, t25654, t25655, t25658, t25660, t25661, t25664, t25672)
}
