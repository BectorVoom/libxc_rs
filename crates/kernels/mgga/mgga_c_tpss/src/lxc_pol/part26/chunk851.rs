//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 851/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk851<F: Float>(t6032: F, t6034: F, t1885: F, t452: F, t6016: F, t1149: F, t1884: F, t1887: F, t473: F, t6017: F, t6019: F, t6022: F, t6024: F, t6027: F, t6031: F, t1889: F, t3154: F) -> (F, F, F, F) {
    let t6035 = t6032 * t6034;
    let t6038 = t1885 * t452 * t6016;
    let t6040 = -t1149 * t6019 - t1884 * t6038 - t1887 * t6022 + t473 * t6017 + 2.0 * t6024 * t6027 - t6031 * t6035;
    let t6044 = t1889 * t3154;
    (t6035, t6038, t6040, t6044)
}
