//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 965/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk965<F: Float>(t10520: F, t14061: F, t14064: F, t14065: F, t14068: F, t14072: F, t14111: F, t14112: F, t14116: F, t7945: F, t7954: F, t7960: F, t7972: F, t7975: F, t8112: F, t8117: F) -> (F,) {
    let t14262 = t14061 + t7945 + t14064 + t14065 + t14068 + t14072 - t7954 - t7960 + t7972 + t7975 + t14111 + t14112 + t10520 + t14116 + t8112 - t8117;
    (t14262,)
}
