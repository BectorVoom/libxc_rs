//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1409/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1409<F: Float>(t16311: F, t3788: F, t6414: F, t6936: F, t1339: F, t20554: F, t20563: F, t221: F, t26284: F, t20442: F, t22833: F, t2002: F, t20595: F, t559: F) -> (F, F, F, F, F) {
    let t107183 = t6936 * t3788 * t16311 * t6414;
    let t107186 = t6936 * t1339 * t20554;
    let t107189 = t26284 * t221 * t20563;
    let t107198 = t22833 * t20442;
    let t107205 = t20595 * t2002 * t559;
    (t107183, t107186, t107189, t107198, t107205)
}
