//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2227/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2227<F: Float>(t4657: F, t6688: F, t7566: F, t82632: F, t23384: F, t25400: F, t13611: F, t13933: F, t13939: F, t14552: F, t1922: F, t1945: F, t23323: F, t23346: F, t23372: F, t23725: F, t25420: F, t25755: F, t25827: F, t3026: F, t3176: F, t388: F, t4557: F, t4694: F, t6687: F, t6689: F, t6690: F, t6691: F, t6776: F, t7562: F, t83329: F) -> F {
    let t88868 = t6688 * t4657;
    let t88882 = t82632 * t7566;
    let t88889 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25400;
    let t88900 = F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t88868 * t6691 + F::cast_from(4.0_f64) * t4557 * t23725 + F::cast_from(4.0_f64) * t3026 * t25420 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6689 * t6690 * t13611 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25827 + F::cast_from(0.18277045187202515961e-2_f64) * t88882 + F::cast_from(4.0_f64) * t14552 * t6776 + F::cast_from(2.0_f64) * t25755 * t3176 - t88889 + F::cast_from(0.80418998823691070228e-1_f64) * t23323 * t7562 - F::cast_from(0.18277045187202515961e-2_f64) * t83329 - F::cast_from(2.0_f64) * t23372 * t4694 + t13939 * t1945 * t388 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t13933 * t1922;
    t88900
}
