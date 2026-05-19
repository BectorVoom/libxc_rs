//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1064/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1064<F: Float>(t2629: F, t4953: F, t5043: F, t9133: F, t10980: F, t11169: F, t11172: F, t14495: F, t14497: F, t14501: F, t14503: F, t14505: F, t14507: F, t8616: F, t8627: F) -> (F, F, F) {
    let t14585 = F::cast_from(0.11696447245269292414e1_f64) * t2629 * t4953;
    let t14586 = t5043 * t9133;
    let t14610 = F::cast_from(0.67094444444444444443e-1_f64) * t14495 + F::cast_from(0.18396666666666666667e-1_f64) * t14497 - F::cast_from(0.13418888888888888889e0_f64) * t8616 - F::cast_from(0.91983333333333333333e-1_f64) * t8627 - F::new(0.11038e0) * t14501 + F::new(0.5519e-1) * t14503 - F::cast_from(0.20128333333333333333e0_f64) * t14505 + F::cast_from(0.10064166666666666667e0_f64) * t14507 - F::cast_from(0.26837777777777777779e0_f64) * t10980 + t11169 + t11172;
    (t14585, t14586, t14610)
}
