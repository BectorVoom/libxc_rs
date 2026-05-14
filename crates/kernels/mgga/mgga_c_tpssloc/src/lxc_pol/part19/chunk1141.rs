//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1141/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1141<F: Float>(t9929: F, t9932: F, t31: F, t717: F, t9898: F, t185: F, t2658: F, t39103: F, t607: F, t707: F, t9862: F, t2250: F, t4194: F, t750: F, t39658: F, t41266: F, t41270: F, t41273: F, t41275: F, t41278: F, t41281: F) -> (F, F, F, F, F, F) {
    let t41282 = t9929 * t9932;
    let t41283 = 144.0 * t41282;
    let t41284 = t31 * t717;
    let t41286 = 96.0 * t41284 * t9898;
    let t41289 = 36.0 * t2658 * t185 * t39103;
    let t41291 = t707 * t9862 * t607;
    let t41292 = 16.0 * t41291;
    let t41295 = t4194 * t750 * t607 * t2250;
    let t41296 = 144.0 * t41295;
    let t41297 = -t41266 + t41270 - t39658 + t41273 + t41275 + t41278 + t41281 + t41283 + t41286 + t41289 + t41292 + t41296;
    (t41283, t41286, t41289, t41292, t41296, t41297)
}
