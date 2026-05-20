//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 830/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk830<F: Float>(t246: F, t9645: F, t2628: F, t835: F, t812: F, t2690: F, t815: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F) -> (F, F, F, F, F) {
    let t9646 = t9645 * t246;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9688 = F::new(1.0) / t126 / t136 * t116 / F::new(4.0);
    let t9689 = t9688 * t16;
    let t9691 = t2386 * t625;
    (t9646, t9667, t9671, t9689, t9691)
}
