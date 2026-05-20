//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 778/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk778<F: Float>(t9384: F, t9385: F, t2341: F, t659: F, t2248: F, t9256: F, t95: F, t101: F, t102: F, t2350: F, t662: F, t2349: F) -> (F, F, F, F, F, F, F, F) {
    let t9386 = t9384 * t9385;
    let t9389 = t2341 * t659;
    let t9390 = t9389 * t2248;
    let t9393 = F::new(3.0) * t9256;
    let t9394 = t95 * t9393;
    let t9397 = t102 * t101;
    let t9398 = F::new(1.0) / t9397;
    let t9399 = t2350 * t662;
    let t9400 = t9398 * t9399;
    let t9403 = t2349 * t662;
    (t9386, t9389, t9390, t9393, t9394, t9398, t9400, t9403)
}
