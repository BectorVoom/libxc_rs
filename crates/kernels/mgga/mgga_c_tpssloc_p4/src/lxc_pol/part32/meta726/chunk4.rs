//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2345/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2345<F: Float>(t5456: F, t7263: F, t2109: F, t96461: F, t96469: F, t96425: F, t22549: F, t24514: F, t24517: F, t26016: F, t27298: F, t83717: F, t85501: F, t90098: F, t90101: F, t90104: F, t96135: F, t96138: F, t96418: F, t96422: F, t96466: F, t96473: F) -> (F, F) {
    let t104729 = t7263 * t5456;
    let t104735 = t2109 * t96461;
    let t104740 = t2109 * t96469;
    let t104749 = t2109 * t96425;
    let t104758 = F::new(35.0) * t85501 * t96418 - F::new(10.0) * t24514 * t96422 - F::new(10.0) / F::new(3.0) * t22549 * t104735 - F::new(5.0) * t24514 * t96466 - F::new(5.0) / F::new(3.0) * t22549 * t104740 - F::new(5.0) / F::new(3.0) * t96473 * t24517 - F::new(10.0) / F::new(3.0) * t26016 * t96135 - F::new(10.0) / F::new(3.0) * t26016 * t96138 + F::new(10.0) * t83717 * t104749 - F::new(10.0) / F::new(3.0) * t90098 * t27298 - F::new(10.0) / F::new(3.0) * t90101 * t27298 - F::new(10.0) / F::new(3.0) * t90104 * t27298;
    (t104729, t104758)
}
