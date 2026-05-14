//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1132/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1132<F: Float>(t205: F, t41160: F, t792: F, t9558: F, t118: F, t794: F, t9458: F, t2576: F, t9516: F, t207: F, t40394: F, t40399: F, t210: F, t214: F, t2571: F, t40848: F, t40972: F, t40977: F, t41142: F, t41144: F, t41149: F, t41151: F, t41155: F, t41156: F, t41158: F, t787: F) -> (F, F) {
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    let t41173 = t41170 * t118 * t794 * t9458;
    let t41181 = t2576 * t118 * t794 * t9516;
    let t41185 = 0.69444444444444444445e-4 * t40394 * t207 * t40399;
    let t41186 = 0.99999999999999999996e-2 * t41142 - 0.79999999999999999997e-1 * t41144 - 0.29999999999999999998e-1 * t41149 + 0.15555555555555555555e-1 * t41151 + t41155 + 0.22469135802469135801e0 * t41156 + 0.18666666666666666665e0 * t41158 + 0.99999999999999999995e-1 * t41161 * t210 * t214 * t40972 + 0.14999999999999999999e-1 * t2571 * t210 * t214 * t40977 + 0.39999999999999999998e-1 * t41173 - 0.16666666666666666666e-2 * t787 * t210 * t214 * t40848 + 0.33333333333333333332e-2 * t41181 - t41185;
    (t41161, t41186)
}
