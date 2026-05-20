//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1155/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1155<F: Float>(t815: F, t836: F, t812: F, t2617: F, t2642: F, t1891: F, t67: F, t246: F, t2628: F, t835: F, t2690: F, t831: F) -> (F, F, F, F, F, F, F) {
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    let t9642 = t2617 * t2642;
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    (t9638, t9642, t9645, t9646, t9667, t9671, t9672)
}
