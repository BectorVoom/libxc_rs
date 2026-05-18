//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1345/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1345<F: Float>(t24574: F, t29551: F, t8003: F, t94490: F, t29694: F, t1170: F, t2121: F, t29670: F, t29678: F, t7280: F, t225: F, t29687: F) -> (F, F, F, F, F, F) {
    let t104504 = t24574 * t29551;
    let t104506 = t94490 * t8003;
    let t104509 = t24574 * t29694;
    let t104521 = t2121 * t1170 * t29670;
    let t104527 = t29678 * t7280;
    let t104556 = t29687 * t225;
    (t104504, t104506, t104509, t104521, t104527, t104556)
}
