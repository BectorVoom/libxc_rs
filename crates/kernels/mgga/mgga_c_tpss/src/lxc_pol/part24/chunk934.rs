//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 934/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk934<F: Float>(t3759: F, t664: F, t3803: F, t673: F, t1421: F, t2202: F, t3750: F) -> (F, F, F, F, F, F) {
    let t10982 = t664 * t3759;
    let t10983 = 0.19931111111111111111e0 * t10982;
    let t10989 = t673 * t3803;
    let t10990 = 0.10954222222222222222e0 * t10989;
    let t10994 = t2202 * t1421;
    let t11002 = t664 * t3750;
    (t10982, t10983, t10989, t10990, t10994, t11002)
}
