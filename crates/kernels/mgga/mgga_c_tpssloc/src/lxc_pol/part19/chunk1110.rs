//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1110/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1110<F: Float>(t40667: F, t67: F, t758: F, t9915: F, t10126: F, t2379: F, t2522: F, t2523: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40622: F, t40627: F, t40663: F, t4314: F, t776: F, t9516: F) -> (F, F, F) {
    let t40668 = 0.20779030926817756511e3 * t40667;
    let t40670 = t9915 * t67 * t758;
    let t40671 = 0.73245789224026180216e-3 * t40670;
    let t40672 = 36.0 * t10126 * t2379 * t4314 + 12.0 * t2522 * t2523 * t9516 + 12.0 * t2522 * t40622 * t776 - t39249 - t39256 - t39309 + t39312 + t39316 + t39320 + t40627 + t40663 - t40668 - t40671;
    (t40668, t40671, t40672)
}
