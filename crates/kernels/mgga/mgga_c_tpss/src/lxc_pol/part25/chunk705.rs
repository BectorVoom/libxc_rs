//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 705/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk705<F: Float>(t1953: F, t1955: F, t1957: F, t1960: F, t1962: F, t1964: F, t1967: F, t1969: F, t1973: F, t1317: F) -> (F, F) {
    let t4566 = t1953 + t1955 + t1957 + t1960 + t1962 + t1964 + t1967 + t1969 + t1973;
    let t4570 = t1317 * t1317;
    (t4566, t4570)
}
