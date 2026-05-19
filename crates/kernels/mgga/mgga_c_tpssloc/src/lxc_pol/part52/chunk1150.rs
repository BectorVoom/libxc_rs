//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1150/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1150<F: Float>(t30694: F, t829: F, t6585: F, t8339: F, t1894: F, t59: F, t776: F, t6591: F, t6600: F, t6599: F, t6612: F, t6605: F) -> (F, F, F, F, F, F, F, F) {
    let t30695 = t30694 * t829;
    let t30697 = t6585 * t8339;
    let t30698 = F::cast_from(0.56521858531796547196e-2_f64) * t30697;
    let t30700 = t1894 * t59 * t776;
    let t30701 = t6591 * t30700;
    let t30703 = t6600 * t8339;
    let t30704 = t6599 * t30703;
    let t30705 = F::cast_from(0.13457585364713463618e-3_f64) * t30704;
    let t30706 = t6612 * t829;
    let t30707 = t6605 * t30706;
    (t30695, t30698, t30700, t30701, t30703, t30705, t30706, t30707)
}
