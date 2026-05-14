//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1040/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1040<F: Float>(t23222: F, t6553: F, t6552: F, t218: F, t23150: F, t212: F, t252: F, t6554: F, t23171: F, t23168: F, t6556: F, t22975: F, t22979: F, t23191: F, t23198: F, t23202: F, t23207: F, t23209: F, t23211: F, t23215: F, t23220: F, t259: F, t2597: F, t2713: F, t6632: F, t6663: F, t855: F) -> (F, F, F, F, F) {
    let t23223 = t6553 * t23222;
    let t23224 = t6552 * t23223;
    let t23226 = t218 * t23150;
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = 0.82246703342411321824e-2 * t23230;
    let t23232 = t23168 * t6556;
    let t23233 = 0.76763589786250567036e-1 * t23232;
    let t23234 = 4.0 * t2713 * t6632 + 2.0 * t855 * t22975 + 4.0 * t855 * t22979 - t855 * t23191 - 2.0 * t2713 * t6663 + 0.16449340668482264365e-1 * t23198 + 4.0 * t2597 * t6632 + t23202 * t259 + t23207 + 0.82246703342411321824e-2 * t23209 + 2.0 * t23211 * t259 - 6.0 * t855 * t23215 - 0.82246703342411321825e-2 * t23220 - 0.16449340668482264365e-1 * t23224 + t23226 * t259 - t23231 + t23233;
    (t23223, t23226, t23228, t23229, t23234)
}
