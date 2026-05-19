//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 959/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk959<F: Float>(t2535: F, t3691: F, t1372: F, t3787: F, t215: F, t535: F, t9569: F, t1314: F, t2559: F, t1317: F, t795: F, t9580: F) -> (F, F, F, F, F, F) {
    let t12142 = t3691 * t2535;
    let t12171 = t3787 * t1372;
    let t12188 = F::cast_from(0.28086419753086419752e-1_f64) * t9569 * t535 * t215;
    let t12189 = t2559 * t1314;
    let t12190 = t12189 * t1317;
    let t12194 = F::cast_from(0.16435185185185185185e-1_f64) * t9580 * t535 * t795;
    (t12142, t12171, t12188, t12189, t12190, t12194)
}
