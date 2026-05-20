//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 585/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk585<F: Float>(t265: F, t394: F, t2057: F, t25: F, t202: F, t2056: F, t193: F, t870: F) -> (F, F, F, F) {
    let t395 = t265 < t394;
    let t2058 = t2057 * t25;
    let t2061 = t202 * t2056;
    let t2063 = t193 * t2061 * t870;
    let t2064 = piecewise3::<F>(t395, F::new(0.0), t2063);
    (t2058, t2061, t2063, t2064)
}
