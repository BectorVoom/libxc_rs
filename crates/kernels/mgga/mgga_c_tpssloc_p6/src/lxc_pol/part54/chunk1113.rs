//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1113/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1113<F: Float>(t52: F, t8027: F, t2136: F, t461: F, t7573: F, t7324: F, t3448: F, t4729: F, t475: F, t5011: F, t68: F, t7328: F) -> (F, F, F, F) {
    let t27680 = t8027 * t52;
    let t27681 = t27680 * t2136;
    let t27683 = t7573 * t461;
    let t27684 = t7324 * t27683;
    let t27687 = t3448 * t4729;
    let t27691 = t5011 * t68 * t475;
    let t27692 = t7328 * t27691;
    (t27681, t27684, t27687, t27692)
}
