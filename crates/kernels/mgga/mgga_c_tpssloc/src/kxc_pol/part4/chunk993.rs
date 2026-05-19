//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 993/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk993<F: Float>(t5499: F, t9929: F, t172: F, t5522: F, t763: F, t184: F, t5398: F, t607: F, t4194: F, t9864: F, t9866: F, t2752: F, t5664: F) -> (F, F, F, F, F, F) {
    let t16612 = F::new(12.0) * t9929 * t5499;
    let t16616 = t5522 * t172;
    let t16617 = t16616 * t763;
    let t16618 = F::cast_from(0.5848223622634646207e0_f64) * t16617;
    let t16619 = t184 * t5398;
    let t16620 = t16619 * t607;
    let t16622 = F::new(12.0) * t4194 * t16620;
    let t16623 = F::cast_from(0.11696447245269292414e1_f64) * t9864;
    let t16624 = F::cast_from(0.17315859105681463759e2_f64) * t9866;
    let t16625 = t5664 * t2752;
    (t16612, t16618, t16622, t16623, t16624, t16625)
}
