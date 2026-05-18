//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1085/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1085<F: Float>(t13335: F, t929: F, t926: F, t140: F, t4969: F, t925: F, t4973: F, t1465: F, t3749: F, t8523: F, t11569: F, t1289: F) -> (F, F, F, F, F) {
    let t14955 = t929 * t13335;
    let t14956 = t926 * t14955;
    let t14959 = t140 * t4969;
    let t14960 = t925 * t14959;
    let t14964 = t140 * t4973;
    let t14965 = t925 * t14964;
    let t14969 = t1465 * t3749;
    let t14970 = t8523 * t14969;
    let t14973 = t11569 * t1289;
    (t14956, t14960, t14965, t14970, t14973)
}
