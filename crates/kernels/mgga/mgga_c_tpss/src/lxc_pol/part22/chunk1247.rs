//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1247/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1247<F: Float>(t19352: F, t5791: F, t18660: F, t6073: F, t1792: F, t18673: F, t19411: F, t5794: F, t62307: F, t62309: F, t62314: F, t62343: F, t62349: F, t65189: F, t65296: F, t65299: F, t65302: F) -> (F,) {
    let t67389 = 16.0 / 9.0 * t19352 * t5791;
    let t67391 = 16.0 / 9.0 * t6073 * t18660;
    let t67407 = -t67389 - t67391 - 880.0 / 27.0 * t62307 - 352.0 / 27.0 * t62309 + 32.0 / 9.0 * t62314 - 160.0 / 9.0 * t62343 - 80.0 / 3.0 * t62349 + 20.0 / 3.0 * t65189 * t18673 - 2.0 / 3.0 * t65296 * t1792 - 4.0 / 3.0 * t65299 * t1792 - 4.0 / 3.0 * t65302 * t1792 - 4.0 / 3.0 * t19411 * t5794;
    (t67407,)
}
