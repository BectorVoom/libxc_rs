//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 764/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk764<F: Float>(t11599: F, t498: F, t14236: F, t14237: F, t2078: F, t11662: F, t14243: F, t11666: F, t14249: F, t1971: F, t2144: F, t495: F, t7230: F, t8946: F, t14125: F, t14131: F, t8431: F) -> (F, F, F, F, F) {
    let t75925 = t11599 * t498;
    let t75928 = t14236 * t14237 * t2078 * t75925;
    let t75932 = t14236 * t14243 * t2078 * t11662;
    let t75936 = t14236 * t14249 * t2078 * t11666;
    let t75943 = t7230 * t1971 * t2144 * t8946 * t495;
    let t75946 = t14131 * t14125 * t8431;
    (t75928, t75932, t75936, t75943, t75946)
}
