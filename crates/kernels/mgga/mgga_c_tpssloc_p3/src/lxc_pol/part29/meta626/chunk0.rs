//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2068/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2068<F: Float>(t3540: F, t7334: F, t11832: F, t2127: F, t2132: F, t2136: F, t2250: F, t24684: F, t7324: F, t7331: F, t23413: F, t461: F) -> (F, F, F, F, F, F) {
    let t86275 = t7334 * t3540;
    let t86278 = F::new(5.0) / F::new(1296.0) * t2127 * t11832;
    let t86282 = t2132 * t2250 * t2136;
    let t86292 = t7324 * t24684;
    let t86293 = t86292 * t7331;
    let t86296 = t7324 * t23413 * t461;
    (t86275, t86278, t86282, t86292, t86293, t86296)
}
