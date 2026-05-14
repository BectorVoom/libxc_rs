//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1293/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293<F: Float>(t1147: F, t1156: F, t1164: F, t43911: F, t11940: F, t11947: F, t1254: F, t193: F, t336: F, t3633: F, t3637: F, t3640: F, t43670: F, t43672: F, t43674: F, t43678: F, t43683: F, t43685: F, t43687: F, t43695: F, t43702: F, t43703: F, t43706: F, t4700: F) -> (F, F) {
    let t43915 = 0.5848223622634646207e0 * t1164 * t1147 * t43911 * t1156;
    let t43920 = -4.0 * t11940 * t1254 * t3640 * t4700 + 12.0 * t11947 * t3633 * t3637 * t4700 - 6.0 * t193 * t336 * t43703 * t43706 - t43670 - t43672 + t43674 - t43678 - t43683 + t43685 - t43687 - t43695 - t43702 - t43915;
    (t43915, t43920)
}
