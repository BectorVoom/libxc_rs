//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 869/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk869<F: Float>(t11588: F, t1184: F, t3451: F, t3447: F, t3448: F, t3475: F, t1239: F, t68: F, t225: F, t3484: F, t1222: F, t3567: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F) -> (F, F, F, F, F, F, F, F) {
    let t11589 = t11588 * t1184;
    let t11590 = t11589 * t3451;
    let t11591 = t3447 * t11590;
    let t11593 = t3448 * t3475;
    let t11604 = t1239 * t1239;
    let t11605 = 1.0 / t11604;
    let t11606 = t68 * t11605;
    let t11613 = t3484 * t225;
    let t11642 = t3567 * t1222;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    (t11589, t11591, t11593, t11606, t11613, t11642, t11644, t11647)
}
