//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 539/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk539<F: Float>(t3439: F, t974: F, t3242: F, t461: F, t2244: F, t337: F, t51: F, t1887: F, t1176: F, t60: F, t1184: F, t1089: F, t460: F, t607: F, t3247: F, t1177: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3440 = t974 * t3439;
    let t3441 = t461 * t3242;
    let t3442 = t3441 * t2244;
    let t3443 = t3440 * t3442;
    let t3446 = t51 * t337;
    let t3447 = t3446 * t1887;
    let t3448 = t60 * t1176;
    let t3449 = t3448 * t1184;
    let t3450 = t460 * t1089;
    let t3451 = t3450 * t607;
    let t3452 = t3449 * t3451;
    let t3455 = t461 * t3247;
    let t3456 = t3455 * t2244;
    let t3457 = t1177 * t3456;
    (t3440, t3441, t3442, t3443, t3447, t3448, t3449, t3450, t3451, t3452, t3456, t3457)
}
