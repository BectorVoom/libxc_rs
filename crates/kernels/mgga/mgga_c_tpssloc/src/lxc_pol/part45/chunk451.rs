//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 451/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk451<F: Float>(t1887: F, t3446: F, t1176: F, t60: F, t1184: F, t1089: F, t460: F, t607: F, t3247: F, t461: F, t2244: F, t1177: F, t1178: F, t2250: F, t3293: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F, F) {
    let t3447 = t3446 * t1887;
    let t3448 = t60 * t1176;
    let t3449 = t3448 * t1184;
    let t3450 = t460 * t1089;
    let t3451 = t3450 * t607;
    let t3452 = t3449 * t3451;
    let t3455 = t461 * t3247;
    let t3456 = t3455 * t2244;
    let t3457 = t1177 * t3456;
    let t3460 = t1178 * t2250;
    let t3461 = t1177 * t3460;
    let t3464 = 5.0 / 18.0 * t3293;
    let t3469 = -t3464 + 2.0 / 9.0 * t3295 + t3299 / 18.0 - t3302 / 3.0 - t3305 / 6.0;
    (t3447, t3452, t3457, t3461, t3469)
}
