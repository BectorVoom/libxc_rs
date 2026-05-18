//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 852/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk852<F: Float>(t214: F, t30689: F, t1880: F, t6585: F, t8339: F, t1894: F, t59: F, t776: F, t6591: F, t6600: F, t6599: F, t6612: F, t829: F) -> (F, F, F, F, F, F, F, F) {
    let t30690 = t214 * t30689;
    let t30692 = F::new(0.16449340668482264365e-1) * t1880 * t30690;
    let t30697 = t6585 * t8339;
    let t30700 = t1894 * t59 * t776;
    let t30701 = t6591 * t30700;
    let t30703 = t6600 * t8339;
    let t30704 = t6599 * t30703;
    let t30706 = t6612 * t829;
    (t30690, t30692, t30697, t30700, t30701, t30703, t30704, t30706)
}
