//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 454/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk454<F: Float>(t1213: F, t3572: F, t478: F, t483: F, t3068: F, t1244: F, t1230: F, t820: F, t1089: F, t415: F, t1236: F, t225: F) -> (F, F, F, F, F) {
    let t3573 = t1213 * t3572;
    let t3575 = t478 * t483;
    let t3576 = t3575 * t3068;
    let t3577 = t1244 * t3576;
    let t3578 = t820 * t1230;
    let t3584 = F::new(1.0) / t415 / t1089;
    let t3593 = t1236 * t225;
    (t3573, t3577, t3578, t3584, t3593)
}
