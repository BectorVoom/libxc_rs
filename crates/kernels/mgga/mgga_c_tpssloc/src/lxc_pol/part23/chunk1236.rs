//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1236/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1236<F: Float>(t76829: F, t76865: F, t76943: F, t76974: F, t225: F, t76634: F, t76636: F, t76641: F, t76643: F, t76647: F, t76652: F, t76654: F, t76657: F, t76659: F, t76661: F, t76663: F) -> (F, F, F) {
    let t76976 = t76829 + t76865 + t76943 + t76974;
    let t76977 = t76976 * t225;
    let t76995 = t76634 - t76636 - t76641 + t76643 + t76647 - t76652 - t76654 + t76657 + t76659 + t76661 + t76663;
    (t76976, t76977, t76995)
}
