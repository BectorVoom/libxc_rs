//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 473/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk473<F: Float>(t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F, t94: F, t102: F, t177: F, t738: F, t745: F, t746: F) -> (F, F, F, F, F, F, F, F) {
    let t2327 = F::new(11.0) / F::new(9.0) * t2281 * t107;
    let t2328 = t626 * t667;
    let t2331 = F::new(1.0) / t655 / t106;
    let t2341 = F::new(1.0) / t94;
    let t2349 = F::new(1.0) / t102;
    let t2367 = t738 * t177;
    let t2368 = F::new(1.0) / t2367;
    let t2369 = t745 * t745;
    let t2371 = t2368 * t2369 * t746;
    (t2327, t2328, t2331, t2341, t2349, t2368, t2369, t2371)
}
