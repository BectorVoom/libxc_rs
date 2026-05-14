//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 839/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk839<F: Float>(t12132: F, t17: F, t3826: F, t592: F, t1285: F, t2225: F, t2371: F, t3691: F, t1294: F, t9494: F, t2535: F, t1372: F, t3787: F, t215: F, t535: F, t9569: F) -> (F, F, F, F, F, F, F, F) {
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12136 = t2225 * t1285;
    let t12138 = t3691 * t2371;
    let t12141 = 0.10254018858216406658e4 * t1294 * t9494;
    let t12142 = t3691 * t2535;
    let t12171 = t3787 * t1372;
    let t12188 = 0.28086419753086419752e-1 * t9569 * t535 * t215;
    (t12133, t12134, t12136, t12138, t12141, t12142, t12171, t12188)
}
