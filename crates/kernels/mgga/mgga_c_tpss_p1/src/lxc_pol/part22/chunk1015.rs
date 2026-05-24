//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1015/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1015<F: Float>(t10982: F, t10353: F, t836: F, t835: F, t128: F) -> (F, F, F) {
    let t10983 = F::cast_from(0.19931111111111111111e0_f64) * t10982;
    let t10984 = t836 * t10353;
    let t10985 = t835 * t10984;
    let t10986 = t128 * t10985;
    (t10983, t10984, t10986)
}
