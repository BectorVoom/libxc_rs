//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1594/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1594<F: Float>(t23230: F, t225: F, t7072: F, t7085: F, t23251: F, t23261: F, t2752: F, t7109: F) -> (F, F, F, F, F, F) {
    let t24291 = F::cast_from(0.16449340668482264365e-1_f64) * t23230;
    let t24297 = t7072 * t225;
    let t24305 = t7085 * t225;
    let t24318 = F::cast_from(0.52089578783527170489e-1_f64) * t23251;
    let t24321 = F::cast_from(0.12793931631041761173e0_f64) * t23261;
    let t24339 = t7109 * t2752;
    (t24291, t24297, t24305, t24318, t24321, t24339)
}
