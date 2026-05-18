//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 367/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk367<F: Float>(t94: F, t102: F, t177: F, t738: F, t745: F, t746: F, t761: F, t118: F, t187: F, t677: F, t763: F, t200: F, t262: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2341 = F::new(1.0) / t94;
    let t2349 = F::new(1.0) / t102;
    let t2367 = t738 * t177;
    let t2368 = F::new(1.0) / t2367;
    let t2369 = t745 * t745;
    let t2371 = t2368 * t2369 * t746;
    let t2373 = F::new(0.11696447245269292414e1) * t761 * t2371;
    let t2374 = t187 * t118;
    let t2375 = t677 * t763;
    let t2377 = F::new(0.10843581300301739842e-1) * t2374 * t2375;
    let t2378 = t200 * t262;
    (t2341, t2349, t2368, t2369, t2371, t2373, t2375, t2377, t2378)
}
