//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 447/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk447<F: Float>(t809: F, t87: F, t820: F, t98: F, t1685: F, t68: F, t131: F, t117: F, t504: F, t325: F, t623: F) -> (F, F, F, F, F, F) {
    let t4861 = t87 * t809;
    let t4882 = t98 * t820;
    let t4961 = t68 * t1685;
    let t4962 = t4961 * t131;
    let t4965 = t504 * t117;
    let t4985 = t623 * t325;
    (t4861, t4882, t4961, t4962, t4965, t4985)
}
