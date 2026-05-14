//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1185/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1185<F: Float>(t5489: F, t62306: F, t18646: F, t5492: F, t31450: F, t5784: F, t18338: F, t5791: F, t1981: F, t1985: F, t68: F, t1791: F, t62020: F, t18351: F, t5790: F, t18350: F) -> (F, F, F, F, F, F, F, F) {
    let t62307 = t62306 * t5489;
    let t62309 = t5492 * t18646;
    let t62311 = t31450 * t5784;
    let t62314 = t18338 * t5791;
    let t62330 = t1981 * t1985 * t68;
    let t62339 = t1791 * t62020;
    let t62342 = t5790 * t18351;
    let t62343 = t18350 * t62342;
    (t62307, t62309, t62311, t62314, t62330, t62339, t62342, t62343)
}
