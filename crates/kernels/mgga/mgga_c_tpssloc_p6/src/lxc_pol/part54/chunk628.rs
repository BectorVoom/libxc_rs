//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 628/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk628<F: Float>(t5010: F, t5051: F, t466: F, t1752: F, t225: F, t1251: F, t1760: F, t3598: F, t1243: F, t5000: F, t1215: F, t3612: F) -> (F, F, F, F, F, F, F) {
    let t5052 = t5010 + t5051;
    let t5053 = t466 * t5052;
    let t5055 = t1752 * t225;
    let t5059 = t1760 * t1251;
    let t5060 = t3598 * t5059;
    let t5064 = t5000 * t1243;
    let t5068 = t3612 * t1215;
    (t5052, t5053, t5055, t5059, t5060, t5064, t5068)
}
