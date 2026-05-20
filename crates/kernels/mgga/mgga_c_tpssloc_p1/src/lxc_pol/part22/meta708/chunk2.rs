//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2304/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2304<F: Float>(t67099: F, t39309: F, t39312: F, t39316: F, t39320: F, t40673: F, t40679: F, t40685: F, t67095: F, t67096: F, t67097: F, t16693: F, t16713: F) -> (F, F, F) {
    let t67100 = F::cast_from(0.5848223622634646207e0_f64) * t67099;
    let t67101 = t67095 - t39309 + t39312 + t39316 + t39320 - t67096 + t40673 - t40679 + t67097 - t40685 - t67100;
    let t67104 = F::new(72.0) * t16693 * t16713;
    (t67100, t67101, t67104)
}
