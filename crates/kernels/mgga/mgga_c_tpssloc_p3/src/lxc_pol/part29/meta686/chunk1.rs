//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2351/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2351<F: Float>(t94223: F, t94236: F, t94257: F, t94272: F, t95970: F, t96228: F, t96232: F, t96274: F, t2174: F, t5363: F, t1404: F, t8110: F) -> (F, F, F) {
    let t96277 = t94223 + t94236 + t94257 + t94272 + t95970 + t96228 + t96232 + t96274;
    let t96281 = F::new(2.0) * t5363 * t2174;
    let t96283 = F::new(2.0) * t8110 * t1404;
    (t96277, t96281, t96283)
}
