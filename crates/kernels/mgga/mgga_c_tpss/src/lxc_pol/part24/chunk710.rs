//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 710/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk710<F: Float>(t116: F, t1338: F, t645: F, t117: F, t3537: F, t1279: F, t1281: F, t1668: F, t1670: F, t4549: F, t547: F, t548: F, t1953: F, t1955: F, t1957: F, t1960: F, t1962: F, t1964: F, t1967: F, t1969: F, t1973: F) -> (F, F, F, F, F) {
    let t4555 = t116 * t1338;
    let t4556 = t4555 * t645;
    let t4559 = t117 * t3537;
    let t4562 = 3.0 * t1279 * t1670 + 3.0 * t1281 * t1668 + t4549 * t548 + 6.0 * t4556 * t547 + 3.0 * t4559 * t547;
    let t4566 = t1953 + t1955 + t1957 + t1960 + t1962 + t1964 + t1967 + t1969 + t1973;
    (t4555, t4556, t4559, t4562, t4566)
}
