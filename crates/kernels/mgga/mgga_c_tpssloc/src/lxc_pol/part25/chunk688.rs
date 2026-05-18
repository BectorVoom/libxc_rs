//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 688/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk688<F: Float>(t2054: F, t259: F, t2597: F, t2713: F, t6557: F, t6569: F, t6574: F, t7067: F, t7069: F, t7072: F, t7085: F, t7087: F, t7092: F, t7107: F, t855: F, t866: F) -> F {
    let t7109 = -t7067 - F::new(0.3289868133696452873e-1) * t6557 - t7069 + F::new(0.16449340668482264365e-1) * t6569 - F::new(0.16449340668482264365e-1) * t6574 + t7072 * t259 + t7085 * t259 - t7087 * t866 - t2597 * t2054 - t2713 * t2054 + F::new(2.0) * t855 * t7092 - t855 * t7107;
    t7109
}
