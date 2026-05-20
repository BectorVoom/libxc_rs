//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 847/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk847<F: Float>(t521: F, t9861: F, t17: F, t1294: F, t9494: F, t1995: F, t68: F, t215: F, t535: F, t9569: F, t1314: F, t2559: F) -> (F, F, F, F, F, F) {
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12141 = F::cast_from(0.10254018858216406658e4_f64) * t1294 * t9494;
    let t12155 = t68 * t1995;
    let t12188 = F::cast_from(0.28086419753086419752e-1_f64) * t9569 * t535 * t215;
    let t12189 = t2559 * t1314;
    (t12132, t12133, t12141, t12155, t12188, t12189)
}
