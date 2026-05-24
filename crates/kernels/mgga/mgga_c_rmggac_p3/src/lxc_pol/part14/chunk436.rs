//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 436/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk436<F: Float>(t1156: F, t140: F, t1190: F, t1215: F, t453: F, t673: F, t1193: F, t1182: F, t209: F, t463: F, t205: F, t1184: F) -> (F, F, F, F, F, F, F) {
    let t4467 = t1156 * t140;
    let t4477 = t1190 * t1215;
    let t4504 = t673 * t453;
    let t4505 = t1193 * t4504;
    let t4510 = t1182 * t209;
    let t4516 = t463 * t463;
    let t4517 = F::new(1.0) / t4516;
    let t4518 = t205 * t4517;
    let t4522 = t1184 * t209;
    (t4467, t4477, t4505, t4510, t4517, t4518, t4522)
}
