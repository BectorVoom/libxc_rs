//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 851/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk851<F: Float>(t1682: F, t6036: F, t3359: F, t11314: F, t11317: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F, t21741: F, t21747: F, t21751: F) -> (F, F, F) {
    let t21854 = t6036 * t1682;
    let t21855 = t21854 * t3359;
    let t21870 = -t11314 - 0.20839e0 * t18512 + 0.34431666666666666666e0 * t18203 - 0.103295e1 * t18219 - 0.51647499999999999999e0 * t18229 + 0.69463333333333333335e-1 * t18494 - 0.41678000000000000001e0 * t18505 - 0.52945875e1 * t21739 + 0.94674375e0 * t21741 - t11317 + 0.68863333333333333332e0 * t14702 + 0.34731666666666666667e0 * t14766 - 0.104195e0 * t21747 + 0.62517e0 * t21751;
    (t21854, t21855, t21870)
}
