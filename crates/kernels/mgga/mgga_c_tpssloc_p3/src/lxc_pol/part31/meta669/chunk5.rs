//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1982/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1982<F: Float>(t29040: F, t814: F, t1509: F, t7823: F, t1499: F, t16805: F, t2051: F, t26654: F, t4162: F, t4291: F, t7839: F, t812: F, t829: F, t84995: F, t87559: F, t92729: F, t92738: F, t92739: F, t92749: F, t92754: F, t98546: F, t98549: F, t98553: F, t98564: F, t98571: F) -> (F, F) {
    let t101694 = t814 * t29040;
    let t101698 = t7823 * t1509;
    let t101705 = F::cast_from(2.0_f64) * t4162 * t7839 - t87559 - t92729 - F::cast_from(0.3289868133696452873e-1_f64) * t98546 + F::cast_from(0.16449340668482264365e-1_f64) * t98549 - F::cast_from(0.16449340668482264365e-1_f64) * t98553 + t92738 - t92739 + F::cast_from(0.76763589786250567037e-1_f64) * t98564 - t812 * t101694 * t829 + t16805 * t2051 - F::cast_from(2.0_f64) * t4291 * t101698 * t829 + F::cast_from(2.0_f64) * t1499 * t26654 + t92749 + t92754 - F::cast_from(0.16449340668482264365e-1_f64) * t98571 - t84995;
    (t101698, t101705)
}
