//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1397/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1397<F: Float>(t3247: F, t460: F, t2244: F, t1176: F, t134: F, t1184: F, t3451: F, t3447: F, t3448: F, t3475: F, t1239: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t11583 = t460 * t3247;
    let t11584 = t11583 * t2244;
    let t11588 = t134 * t1176;
    let t11589 = t11588 * t1184;
    let t11590 = t11589 * t3451;
    let t11591 = t3447 * t11590;
    let t11593 = t3448 * t3475;
    let t11604 = t1239 * t1239;
    let t11605 = F::new(1.0) / t11604;
    let t11606 = t68 * t11605;
    (t11583, t11584, t11588, t11589, t11591, t11593, t11606)
}
