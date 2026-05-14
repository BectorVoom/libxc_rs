//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1111/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1111<F: Float>(t718: F, t9862: F, t157: F, t182: F, t40661: F, t39264: F, t761: F, t2663: F, t9901: F, t2531: F, t9905: F, t39259: F, t2250: F, t2517: F, t707: F, t751: F, t9449: F) -> (F, F, F, F, F, F, F, F) {
    let t40673 = t718 * t9862;
    let t40674 = 4.0 * t40673;
    let t40677 = 0.19751673498613801407e-1 * t40661 * t157 * t182;
    let t40679 = 0.61524113149298439947e4 * t761 * t39264;
    let t40680 = t9901 * t2663;
    let t40681 = 0.14649157844805236043e-2 * t40680;
    let t40682 = t2531 * t9905;
    let t40683 = 0.14035736694323150897e2 * t40682;
    let t40685 = 0.69263436422725855036e2 * t761 * t39259;
    let t40687 = t707 * t2517 * t2250;
    let t40688 = 24.0 * t40687;
    let t40689 = t9449 * t751;
    (t40674, t40677, t40679, t40681, t40683, t40685, t40688, t40689)
}
