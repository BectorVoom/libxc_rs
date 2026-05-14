//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1214/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1214<F: Float>(t3259: F, t41590: F, t12828: F, t1659: F, t3202: F, t3245: F, t13223: F, t196: F, t197: F, t13220: F, t93: F, t1206: F, t13965: F, t1799: F, t2105: F, t5815: F, t645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65878 = t41590 * t3259;
    let t65882 = t12828 * t3259;
    let t65899 = t1659 * t3202;
    let t65924 = t1659 * t3245;
    let t65941 = t13223 * t196 * t197;
    let t65956 = t93 * t13220;
    let t66051 = t13965 * t1206;
    let t66195 = t2105 * t1799;
    let t66199 = t645 * t5815;
    (t65878, t65882, t65899, t65924, t65941, t65956, t66051, t66195, t66199)
}
