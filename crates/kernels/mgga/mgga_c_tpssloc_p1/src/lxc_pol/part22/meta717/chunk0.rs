//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2325/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2325<F: Float>(t20816: F, t2427: F, t46369: F, t46371: F, t46376: F, t58984: F, t41259: F, t46433: F, t39593: F, t41254: F, t41258: F, t41262: F, t46336: F, t67472: F, t67475: F, t67478: F, t67480: F, t67482: F) -> (F, F, F, F, F, F, F, F) {
    let t67484 = F::new(4.0) * t2427 * t20816;
    let t67485 = F::new(12.0) * t46369;
    let t67486 = F::cast_from(0.65061487801810439052e-1_f64) * t46371;
    let t67487 = F::cast_from(0.17544670867903938621e1_f64) * t46376;
    let t67488 = F::cast_from(0.73245789224026180216e-3_f64) * t58984;
    let t67489 = F::cast_from(0.5848223622634646207e0_f64) * t41259;
    let t67490 = F::cast_from(0.17090684152272775384e-2_f64) * t46433;
    let t67491 = t67472 + t67475 + t67478 + t67480 + t46336 - t39593 + t67482 + t67484 + t67485 - t67486 + t41254 - t67487 + t67488 - t41258 - t67489 - t41262 - t67490;
    (t67484, t67485, t67486, t67487, t67488, t67489, t67490, t67491)
}
