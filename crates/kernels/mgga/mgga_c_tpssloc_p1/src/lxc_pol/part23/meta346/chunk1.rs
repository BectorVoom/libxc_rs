//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1139/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1139<F: Float>(t10021: F, t1336: F, t1339: F, t2690: F, t3788: F, t67: F, t6924: F, t246: F, t39037: F, t522: F, t2221: F, t3824: F) -> (F, F, F, F, F) {
    let t40123 = t1336 * t1339 * t10021;
    let t40159 = t1336 * t3788 * t2690;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40224 = F::cast_from(840.0_f64) * t39037 * t522;
    let t40227 = t2221 * t3824;
    (t40123, t40159, t40168, t40224, t40227)
}
