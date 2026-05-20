//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1665/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1665<F: Float>(t23956: F, t24446: F, t3: F, t112: F, t7222: F, t111: F, t2098: F, t671: F, t7056: F, t2039: F, t2363: F, t12521: F, t12524: F, t1401: F, t16535: F, t2319: F, t23917: F, t3938: F, t3941: F, t577: F, t7230: F, t7235: F) -> (F, F, F, F, F, F, F) {
    let t24447 = t23956 + t24446;
    let t24448 = t3 * t24447;
    let t24462 = t7222 * t112;
    let t24465 = t2098 * t111;
    let t24478 = t7056 * t671;
    let t24481 = t2039 * t2363;
    let t24486 = F::new(0.45e1) * t24447 * t577 + F::new(27.0) * t24462 * t671 + F::new(27.0) * t24465 * t2319 + F::new(0.135e2) * t7230 * t2363 + F::new(0.135e2) * t12521 * t2039 + F::new(54.0) * t12524 * t7235 + F::new(27.0) * t3938 * t7056 + F::new(27.0) * t16535 * t2039 + F::new(54.0) * t3941 * t24478 + F::new(27.0) * t3941 * t24481 + F::new(0.135e2) * t1401 * t23917;
    (t24447, t24448, t24462, t24465, t24478, t24481, t24486)
}
