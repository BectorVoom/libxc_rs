//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1270/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1270<F: Float>(t61065: F, t61073: F, t61080: F, t63949: F, t63951: F, t63953: F, t63955: F, t63957: F, t63961: F, t63962: F, t63964: F, t63967: F, t63968: F, t10674: F, t5559: F, t17960: F, t3667: F) -> (F, F, F) {
    let t63970 = -t63949 + 7.0 / 144.0 * t61065 - t63951 / 96.0 + t63953 / 384.0 + t63955 / 768.0 - 35.0 / 216.0 * t63957 - 7.0 / 48.0 * t61073 + t63961 - t63962 / 384.0 - 119.0 / 1728.0 * t63964 - t61080 + t63967 - t63968 / 48.0;
    let t63971 = t5559 * t10674;
    let t63973 = t17960 * t3667;
    (t63970, t63971, t63973)
}
