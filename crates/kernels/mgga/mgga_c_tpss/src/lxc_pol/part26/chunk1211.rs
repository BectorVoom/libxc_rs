//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1211/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1211<F: Float>(t1586: F, t6016: F, t6025: F, t1880: F, t4322: F, t1561: F, t6032: F, t1143: F, t1148: F, t19144: F, t4303: F, t19150: F, t6521: F, t342: F, t4245: F, t450: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20882 = t6016 * t1586;
    let t20883 = t6025 * t20882;
    let t20886 = t1880 * t4322;
    let t20887 = t6025 * t20886;
    let t20891 = t6032 * t1561;
    let t20892 = t1143 * t1148;
    let t20893 = t20891 * t20892;
    let t20896 = t19144 * t1561;
    let t20897 = t20896 * t4303;
    let t20900 = t19150 * t6521;
    let t20903 = t4245 * t342 * t450;
    (t20883, t20887, t20891, t20892, t20893, t20896, t20897, t20900, t20903)
}
