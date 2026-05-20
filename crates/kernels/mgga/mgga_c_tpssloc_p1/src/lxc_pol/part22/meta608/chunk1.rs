//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2135/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2135<F: Float>(t3242: F, t457: F, t2394: F, t4734: F, t1654: F, t9698: F) -> (F, F, F, F) {
    let t50822 = t457 * t3242;
    let t50826 = t2394 * t4734;
    let t50827 = F::cast_from(0.40256666666666666668e0_f64) * t50826;
    let t50834 = t9698 * t1654;
    (t50822, t50826, t50827, t50834)
}
