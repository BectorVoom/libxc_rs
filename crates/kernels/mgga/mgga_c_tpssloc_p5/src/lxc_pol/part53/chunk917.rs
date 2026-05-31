//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 917/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk917<F: Float>(t112: F, t33915: F, t1458: F, t2039: F, t27188: F, t32235: F, t33152: F, t33154: F, t33234: F, t33893: F, t7042: F, t7801: F, t8446: F, t9012: F) -> (F, F) {
    let t33916 = t33915 * t112;
    let t33928 = F::cast_from(2.0_f64) * t1458 * t32235 + F::cast_from(4.0_f64) * t2039 * t27188 + F::cast_from(4.0_f64) * t2039 * t33234 + F::cast_from(4.0_f64) * t7042 * t7801 + F::cast_from(4.0_f64) * t7801 * t9012 + t33152 + t33154 + F::cast_from(2.0_f64) * t33893 + t33916 + t8446;
    (t33916, t33928)
}
