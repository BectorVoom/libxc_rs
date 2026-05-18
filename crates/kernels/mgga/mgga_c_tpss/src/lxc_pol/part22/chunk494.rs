//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 494/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk494<F: Float>(t1953: F, t1955: F, t1957: F, t1960: F, t1962: F, t1964: F, t1967: F, t1969: F, t1973: F, t574: F, t577: F) -> (F, F) {
    let t1974 = t1953 - t1955 + t1957 + t1960 - t1962 + t1964 + t1967 - t1969 + t1973;
    let t1976 = t574 * t577;
    (t1974, t1976)
}
