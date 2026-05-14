//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 903/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk903<F: Float>(t5575: F, t68: F, t4234: F, t4295: F, t12850: F, t12860: F, t16577: F, t16578: F, t16581: F, t16582: F, t16588: F, t16612: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9724: F) -> (F, F, F) {
    let t16673 = t5575 * t68;
    let t16679 = t4295 * t4234;
    let t16684 = t12850 + t16577 + t16578 - t9457 + t16581 - t12860 + t16582 - t9469 + t16588 + t9476 + t9484 - t9496 - t9715 + t16612 + t9724;
    (t16673, t16679, t16684)
}
