//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 917/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk917<F: Float>(t1041: F, t10489: F, t3103: F, t3109: F, t3114: F, t376: F, t676: F, t1023: F, t248: F, t1020: F, t1017: F, t3087: F) -> (F, F, F, F, F, F) {
    let t10490 = t1041 * t10489;
    let t10496 = t3109 * t3103;
    let t10504 = t3114 * t3103;
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10515 = t3087 * t1017;
    (t10490, t10496, t10504, t10508, t10511, t10515)
}
