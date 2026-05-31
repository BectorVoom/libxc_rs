//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 450/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk450<F: Float>(t2098: F, t3: F, t1401: F, t2039: F, t577: F, t50: F, t56: F, t63: F, t67: F) -> (F, F, F, F) {
    let t2099 = t3 * t2098;
    let t2105 = F::cast_from(0.45e1_f64) * t2098 * t577 + F::cast_from(0.135e2_f64) * t1401 * t2039;
    let t2108 = t50 * t56 - t63;
    let t2109 = t2108 * t67;
    (t2099, t2105, t2108, t2109)
}
