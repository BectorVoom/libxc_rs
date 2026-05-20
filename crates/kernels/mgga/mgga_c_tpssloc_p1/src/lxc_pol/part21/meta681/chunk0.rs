//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2491/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2491<F: Float>(t41115: F, t4250: F, t4166: F, t9637: F, t2649: F, t13257: F, t2617: F, t4184: F, t4257: F, t9993: F, t13176: F, t2638: F) -> (F, F, F, F, F, F) {
    let t46649 = t41115 * t4250;
    let t46657 = t4166 * t9637;
    let t46658 = t46657 * t2649;
    let t46661 = t2617 * t13257 * t4184;
    let t46663 = t9993 * t4257;
    let t46667 = t13176 * t2638;
    (t46649, t46657, t46658, t46661, t46663, t46667)
}
