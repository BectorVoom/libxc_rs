//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 693/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk693<F: Float>(t2617: F, t2642: F, t1891: F, t67: F, t246: F, t232: F, t2379: F, t2646: F, t2645: F, t2647: F, t9626: F, t210: F, t2553: F, t804: F, t2631: F, t828: F) -> (F, F, F, F, F) {
    let t9642 = t2617 * t2642;
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9649 = t9646 * t2646 * t9647;
    let t9653 = t2645 * t9626 * t2647;
    let t9657 = t210 * t804 * t2553;
    let t9660 = t2631 * t828;
    (t9642, t9649, t9653, t9657, t9660)
}
