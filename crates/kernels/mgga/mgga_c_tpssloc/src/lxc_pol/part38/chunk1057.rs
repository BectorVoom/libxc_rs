//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1057/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1057<F: Float>(t9902: F, t2535: F, t4199: F, t1471: F, t32: F, t2659: F, t9910: F, t4095: F, t67: F, t758: F, t9922: F, t118: F, t1474: F) -> (F, F, F, F, F, F, F) {
    let t13112 = F::new(0.18311447306006545054e-3) * t9902;
    let t13113 = t4199 * t2535;
    let t13114 = F::new(0.5848223622634646207e0) * t13113;
    let t13115 = t32 * t1471;
    let t13117 = F::new(12.0) * t13115 * t2659;
    let t13118 = F::new(4.0) * t9910;
    let t13119 = t4095 * t67;
    let t13121 = F::new(0.36622894612013090108e-3) * t13119 * t758;
    let t13122 = F::new(0.11696447245269292414e1) * t9922;
    let t13123 = t1474 * t118;
    (t13112, t13114, t13117, t13118, t13121, t13122, t13123)
}
