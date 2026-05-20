//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1823/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1823<F: Float>(t2031: F, t83718: F, t2240: F, t240: F, t33: F, t6492: F, t2244: F, t63: F, t23993: F, t6495: F, t1860: F, t22489: F, t7031: F) -> (F, F, F, F, F, F) {
    let t84237 = t2031 * t83718;
    let t84241 = t2240 * t33 * t240;
    let t84242 = t84241 * t6492;
    let t84245 = t2240 * t2244 * t63;
    let t84248 = t6495 * t23993;
    let t84270 = t1860 * t7031 * t22489;
    (t84237, t84241, t84242, t84245, t84248, t84270)
}
