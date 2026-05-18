//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1042/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1042<F: Float>(t3819: F, t876: F, t1429: F, t2574: F, t10982: F, t10989: F, t11049: F, t10992: F, t10994: F, t11041: F, t11044: F, t11047: F, t11051: F, t8647: F, t8871: F, t8872: F) -> (F, F, F, F) {
    let t11289 = t3819 * t876;
    let t11294 = t1429 * t2574;
    let t11309 = F::new(0.34431666666666666666e0) * t10982;
    let t11312 = F::new(0.13892666666666666667e0) * t10989;
    let t11319 = F::new(0.27785333333333333334e0) * t11049;
    let t11321 = t11312 - F::new(0.104195e0) * t10992 - F::new(0.11577222222222222222e0) * t10994 - F::new(0.13892666666666666667e0) * t8647 - t8871 - t8872 + F::new(0.3529725e1) * t11041 - F::new(0.62517e0) * t11044 + F::new(0.20839e0) * t11047 - t11319 + F::new(0.46308888888888888889e-1) * t11051;
    (t11289, t11294, t11309, t11321)
}
