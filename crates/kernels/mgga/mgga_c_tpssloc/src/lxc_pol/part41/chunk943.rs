//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 943/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk943<F: Float>(t5499: F, t9929: F, t172: F, t5522: F, t763: F, t184: F, t5398: F, t607: F, t4194: F, t9864: F, t9866: F, t2752: F, t5664: F, t12943: F, t4101: F, t4205: F) -> (F, F, F, F, F, F, F, F) {
    let t16612 = 12.0 * t9929 * t5499;
    let t16616 = t5522 * t172;
    let t16617 = t16616 * t763;
    let t16618 = 0.5848223622634646207e0 * t16617;
    let t16619 = t184 * t5398;
    let t16620 = t16619 * t607;
    let t16622 = 12.0 * t4194 * t16620;
    let t16623 = 0.11696447245269292414e1 * t9864;
    let t16624 = 0.17315859105681463759e2 * t9866;
    let t16625 = t5664 * t2752;
    let t16629 = 0.23392894490538584828e1 * t12943;
    let t16630 = t4205 * t4101;
    (t16612, t16618, t16622, t16623, t16624, t16625, t16629, t16630)
}
