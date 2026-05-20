//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2270/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2270<F: Float>(t11583: F, t17691: F, t3428: F, t6109: F, t1174: F, t6146: F, t698: F, t6140: F, t18321: F, t3435: F, t15281: F, t18563: F) -> (F, F, F, F, F, F) {
    let t64870 = t11583 * t17691;
    let t64878 = t6109 * t3428;
    let t64881 = t1174 * t698 * t6146;
    let t64885 = t1174 * t698 * t6140;
    let t64951 = t18321 * t3435;
    let t64969 = t1174 * t15281 * t18563;
    (t64870, t64878, t64881, t64885, t64951, t64969)
}
