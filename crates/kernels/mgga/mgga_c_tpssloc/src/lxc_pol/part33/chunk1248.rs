//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1248/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1248<F: Float>(t23132: F, t4166: F, t1516: F, t81763: F, t25064: F, t81788: F, t2693: F, t7503: F, t25132: F, t81882: F, t7500: F, t81911: F) -> (F, F, F, F, F, F) {
    let t87340 = t4166 * t23132;
    let t87345 = t81763 * t1516;
    let t87387 = t81788 * t25064;
    let t87403 = t7503 * t2693;
    let t87405 = t81882 * t25132;
    let t87432 = t81911 * t7500;
    (t87340, t87345, t87387, t87403, t87405, t87432)
}
