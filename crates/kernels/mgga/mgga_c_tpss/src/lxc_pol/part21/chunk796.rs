//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 796/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk796<F: Float>(t1613: F, t72: F, t732: F, t177: F, t737: F, t3200: F, t3301: F, t2292: F, t2302: F, t2310: F, t3198: F, t3209: F, t3213: F, t3281: F, t3307: F, t3310: F) -> (F, F, F, F, F, F, F) {
    let t4435 = t1613 * t72;
    let t4436 = t4435 * t732;
    let t4437 = 0.18311447306006545054e-3 * t4436;
    let t4438 = t1613 * t177;
    let t4439 = t4438 * t737;
    let t4440 = 0.5848223622634646207e0 * t4439;
    let t4441 = 4.0 * t3200;
    let t4442 = 4.0 * t3301;
    let t4443 = -t4437 - t4440 + t3198 - t4441 + t2310 - t3209 - t3213 - t4442 + t3307 + t3281 + t3310 - t2292 + t2302;
    (t4435, t4437, t4438, t4440, t4441, t4442, t4443)
}
