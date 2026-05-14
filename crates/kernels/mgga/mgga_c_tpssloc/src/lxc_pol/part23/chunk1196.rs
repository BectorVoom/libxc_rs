//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1196/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196<F: Float>(t58984: F, t46433: F, t46439: F, t1409: F, t4194: F, t67469: F, t59013: F, t12939: F, t16716: F, t5398: F, t59028: F, t145: F, t185: F, t75929: F, t39658: F, t41258: F, t41262: F, t76024: F) -> (F, F, F, F, F, F, F, F, F) {
    let t76025 = 0.14649157844805236043e-2 * t58984;
    let t76026 = 0.22787578869697033845e-2 * t46433;
    let t76027 = 4.0 * t46439;
    let t76030 = 48.0 * t4194 * t67469 * t1409;
    let t76031 = 72.0 * t59013;
    let t76034 = 144.0 * t12939 * t16716 * t5398;
    let t76035 = 0.10389515463408878255e3 * t59028;
    let t76037 = t145 * t75929 * t185;
    let t76038 = t76024 + t76025 - t41258 - t41262 - t76026 + t76027 + t76030 - t39658 + t76031 + t76034 - t76035 + t76037;
    (t76025, t76026, t76027, t76030, t76031, t76034, t76035, t76037, t76038)
}
