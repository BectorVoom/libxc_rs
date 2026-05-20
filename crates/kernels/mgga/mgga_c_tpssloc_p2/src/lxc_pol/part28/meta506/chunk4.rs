//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1754/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1754<F: Float>(t12524: F, t1401: F, t1458: F, t16521: F, t16524: F, t20173: F, t2039: F, t24462: F, t24465: F, t27170: F, t27240: F, t27254: F, t27273: F, t27276: F, t27281: F, t3938: F, t3941: F, t4072: F, t5371: F, t5376: F, t577: F, t671: F, t7056: F, t7230: F, t7235: F, t7801: F, t7956: F) -> F {
    let t27286 = F::new(0.45e1) * t27240 * t577 + F::new(0.135e2) * t27254 * t671 + F::new(0.135e2) * t24462 * t1458 + F::new(27.0) * t24465 * t5376 + F::new(0.135e2) * t7230 * t4072 + F::new(0.135e2) * t16521 * t2039 + F::new(27.0) * t16524 * t7235 + F::new(0.135e2) * t5371 * t7056 + F::new(27.0) * t12524 * t7956 + F::new(27.0) * t20173 * t7956 + F::new(27.0) * t3941 * t27273 + F::new(27.0) * t3941 * t27276 + F::new(0.135e2) * t3938 * t7801 + F::new(27.0) * t3941 * t27281 + F::new(0.135e2) * t1401 * t27170;
    t27286
}
