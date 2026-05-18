//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 814/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk814<F: Float>(t1238: F, t1761: F, t2121: F, t2124: F, t2155: F, t4945: F, t498: F, t5055: F, t7282: F, t7283: F, t7351: F, t7999: F, t8003: F, t8006: F, t8011: F, t8015: F, t8018: F, t8055: F, t8061: F, t8088: F) -> F {
    let t8090 = -F::new(0.21932454224643019153e-1) * t7999 * t2124 + t7282 - F::new(0.27415567780803773942e-2) * t7283 * t8003 - F::new(0.82246703342411321825e-2) * t7283 * t8006 + F::new(0.82246703342411321825e-2) * t2121 * t8011 - F::new(0.82246703342411321825e-2) * t7283 * t8015 + t8018 * t498 + t8055 * t498 - t7351 * t1761 - t4945 * t2155 - t5055 * t2155 + F::new(2.0) * t1238 * t8061 - t1238 * t8088;
    t8090
}
