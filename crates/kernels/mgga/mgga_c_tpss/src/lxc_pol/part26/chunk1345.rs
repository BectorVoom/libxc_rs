//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1345/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1345<F: Float>(t1860: F, t69152: F, t19345: F, t6471: F, t69198: F, t69206: F, t18350: F, t19232: F, t19235: F, t19342: F, t19349: F, t62019: F, t63495: F, t67935: F, t67938: F, t68127: F, t69143: F, t69147: F, t69195: F, t69203: F, t69210: F) -> (F,) {
    let t73057 = t1860 * t69152;
    let t73062 = t6471 * t19345;
    let t73067 = t1860 * t69198;
    let t73072 = t1860 * t69206;
    let t73081 = 35.0 * t63495 * t69143 - 10.0 * t19232 * t69147 + 10.0 * t62019 * t73057 - 10.0 * t68127 * t19342 - 10.0 / 3.0 * t18350 * t73062 - 10.0 * t19232 * t69195 - 10.0 / 3.0 * t18350 * t73067 - 5.0 * t19232 * t69203 - 5.0 / 3.0 * t18350 * t73072 - 5.0 / 3.0 * t69210 * t19235 - 10.0 / 3.0 * t19349 * t67935 - 10.0 / 3.0 * t19349 * t67938;
    (t73081,)
}
