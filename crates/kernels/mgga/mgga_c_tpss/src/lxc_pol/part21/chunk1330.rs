//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1330/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1330<F: Float>(t3482: F, t619: F, t77: F, t1313: F, t2049: F, t42178: F, t5486: F, t10292: F, t18341: F, t18335: F, t18338: F, t18342: F, t18366: F, t19369: F, t19372: F, t19377: F, t19381: F, t19388: F, t5487: F, t5489: F, t5492: F, t6091: F) -> (F,) {
    let t65321 = t77 * t3482 * t619;
    let t65325 = t77 * t1313 * t2049;
    let t65336 = t42178 * t5486;
    let t65339 = t10292 * t18341;
    let t65342 = 2.0 / 3.0 * t5492 * t19369 + 2.0 / 3.0 * t5492 * t19372 + 5.0 / 3.0 * t18335 * t19388 + 5.0 / 3.0 * t18342 * t19388 + 5.0 / 3.0 * t5487 * t65321 + 5.0 / 6.0 * t5487 * t65325 + 2.0 / 3.0 * t18338 * t6091 + t18366 * t6091 / 3.0 + 2.0 / 3.0 * t5492 * t19377 + 2.0 / 3.0 * t5492 * t19381 + 5.0 / 3.0 * t65336 * t5489 + 5.0 / 3.0 * t65339 * t5489;
    (t65342,)
}
