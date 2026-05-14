//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 653/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk653<F: Float>(t5685: F, t882: F, t123: F, t2765: F, t4335: F, t5679: F, t5683: F, t291: F, t1557: F, t4354: F, t1556: F) -> (F, F, F, F, F, F) {
    let t5686 = t882 * t5685;
    let t5687 = t123 * t5686;
    let t5689 = t2765 + 0.11872222222222222222e-1 * t4335 - 0.11872222222222222222e-1 * t5679 + 0.35616666666666666666e-1 * t5683 - 0.17808333333333333333e-1 * t5687;
    let t5691 = 0.621814e-1 * t5689 * t291;
    let t5693 = 2.0 * t4354 * t1557;
    let t5694 = t1556 * t1556;
    (t5686, t5687, t5689, t5691, t5693, t5694)
}
