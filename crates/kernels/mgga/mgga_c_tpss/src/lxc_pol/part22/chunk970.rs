//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 970/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk970<F: Float>(t1289: F, t7771: F, t2033: F, t3431: F, t7780: F, t2040: F, t10353: F, t1985: F, t1992: F, t3472: F, t3477: F, t581: F, t608: F, t612: F) -> F {
    let t10388 = t7771 * t1289;
    let t10391 = t2033 * t3431;
    let t10398 = t7780 * t1289;
    let t10401 = t2040 * t3431;
    let t10408 = -F::new(280.0) / F::new(27.0) * t10388 * t1985 + F::new(56.0) / F::new(9.0) * t10391 * t581 + F::new(28.0) / F::new(9.0) * t3472 * t1992 - F::new(4.0) / F::new(3.0) * t608 * t10353 + F::new(280.0) / F::new(27.0) * t10398 * t1985 + F::new(56.0) / F::new(9.0) * t10401 * t581 + F::new(28.0) / F::new(9.0) * t3477 * t1992 + F::new(4.0) / F::new(3.0) * t612 * t10353;
    t10408
}
