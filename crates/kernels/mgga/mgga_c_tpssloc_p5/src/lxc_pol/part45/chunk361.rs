//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 361/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk361<F: Float>(t2098: F, t3: F, t1401: F, t2039: F, t577: F, t11: F, t2: F, t584: F, t16: F, t9: F, t587: F, t591: F) -> (F, F, F, F, F, F, F, F) {
    let t2099 = t3 * t2098;
    let t2105 = F::cast_from(0.45e1_f64) * t2098 * t577 + F::cast_from(0.135e2_f64) * t1401 * t2039;
    let t2218 = F::cast_from(0.174e1_f64) * t11;
    let t2219 = t2 * t584;
    let t2220 = F::cast_from(0.696e1_f64) * t2219;
    let t2221 = t9 * t16;
    let t2222 = F::cast_from(0.1122e2_f64) * t2221;
    let t2223 = t587 * t591;
    (t2099, t2105, t2218, t2219, t2220, t2221, t2222, t2223)
}
