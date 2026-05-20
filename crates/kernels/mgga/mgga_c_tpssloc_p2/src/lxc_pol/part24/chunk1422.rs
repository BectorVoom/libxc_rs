//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1422/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1422<F: Float>(t2303: F, t645: F, t72: F, t39049: F, t6489: F, t2240: F, t2251: F, t2261: F, t43: F, t2267: F, t614: F, t38: F, t9287: F) -> (F, F, F, F, F, F) {
    let t83771 = t72 * t2303 * t645;
    let t83775 = t39049 * t6489;
    let t83778 = t2240 * t2251;
    let t83788 = t2261 * t43;
    let t83791 = t614 * t2267;
    let t83796 = t38 * t9287;
    (t83771, t83775, t83778, t83788, t83791, t83796)
}
