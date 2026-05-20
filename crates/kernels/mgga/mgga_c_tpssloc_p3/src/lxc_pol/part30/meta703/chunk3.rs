//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2288/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2288<F: Float>(t1409: F, t1597: F, t23562: F, t343: F, t40: F, t5836: F, t99645: F, t18041: F, t23419: F, t17649: F, t17998: F, t6747: F, t7583: F, t83025: F, t83028: F, t88348: F, t88479: F, t88488: F) -> (F, F, F) {
    let t99660 = t1409 * t1597;
    let t99662 = t23562 * t99660 * t343;
    let t99665 = t40 * t5836;
    let t99667 = t23562 * t99665 * t343;
    let t99671 = t23562 * t99645 * t343;
    let t99680 = t23419 * t18041;
    let t99682 = F::new(5.0) / F::new(6912.0) * t23419 * t17998 - F::cast_from(0.20186378047070195428e-3_f64) * t99662 * t6747 - F::cast_from(0.10093189023535097714e-3_f64) * t99667 * t6747 - F::cast_from(0.10093189023535097714e-3_f64) * t99671 * t6747 + F::cast_from(0.16149102437656156342e-2_f64) * t88348 * t7583 - t23419 * t17649 / F::new(1152.0) + t83025 / F::new(162.0) + t83028 - t88479 / F::new(3456.0) + t88488 + t99680 / F::new(1728.0);
    (t99660, t99665, t99682)
}
