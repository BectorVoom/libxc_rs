//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 508/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk508<F: Float>(t1965: F, t27: F, t567: F, t571: F, t21: F, t25: F, t1953: F, t1955: F, t1957: F, t1960: F, t1962: F, t1964: F, t574: F, t577: F) -> (F, F, F, F, F, F) {
    let t1967 = 30.0 * t1965 * t27;
    let t1969 = 72.0 * t567 * t571;
    let t1970 = t21 * t21;
    let t1971 = 1.0 / t1970;
    let t1973 = 42.0 * t25 * t1971;
    let t1974 = t1953 - t1955 + t1957 + t1960 - t1962 + t1964 + t1967 - t1969 + t1973;
    let t1976 = t574 * t577;
    (t1967, t1970, t1971, t1973, t1974, t1976)
}
