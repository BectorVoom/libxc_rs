//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1282/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1282<F: Float>(t26504: F, t8607: F, t12524: F, t33656: F, t27254: F, t6534: F, t120833: F, t8657: F, t31814: F, t33185: F, t31817: F, t1873: F, t94127: F, t120849: F, t75795: F, t100993: F, t7769: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t122758 = t8607 * t26504;
    let t122776 = 27.0 * t12524 * t33656;
    let t122780 = 0.135e2 * t27254 * t6534;
    let t122784 = 27.0 * t120833 * t8657;
    let t122786 = 27.0 * t33185 * t31814;
    let t122788 = 27.0 * t33185 * t31817;
    let t122790 = 0.135e2 * t94127 * t1873;
    let t122794 = 27.0 * t120849 * t8657;
    let t122800 = 27.0 * t75795 * t8657;
    let t122804 = 27.0 * t100993 * t7769;
    (t122758, t122776, t122780, t122784, t122786, t122788, t122790, t122794, t122800, t122804)
}
