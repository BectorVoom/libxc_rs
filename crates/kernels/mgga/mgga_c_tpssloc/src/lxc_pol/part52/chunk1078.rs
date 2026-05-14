//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1078/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1078<F: Float>(t5: F, t8662: F, t9239: F, t131: F, t7245: F, t2240: F, t7254: F, t8301: F, t31019: F, t31677: F, t31684: F, t31693: F, t31857: F, t8515: F, t8663: F, t112: F, t111: F, t8666: F) -> (F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t31860 = t9239 * t8662;
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    let t31867 = t8301 * t7254;
    let t31868 = t2240 * t31867;
    let t31876 = piecewise3(t8, 0.0, 5.0 / 144.0 * t31857 * t8515 - 5.0 / 24.0 * t31860 * t31677 - 5.0 / 36.0 * t31864 * t31684 + 5.0 / 144.0 * t31868 * t8515 + 5.0 / 72.0 * t8663 * t31693 + 5.0 / 144.0 * t8663 * t31019);
    let t31877 = t31876 * t112;
    let t31880 = t8666 * t111;
    (t31860, t31863, t31864, t31867, t31868, t31876, t31877, t31880)
}
