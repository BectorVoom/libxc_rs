//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1230/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1230<F: Float>(t1143: F, t1148: F, t20891: F, t1561: F, t19144: F, t4303: F, t19150: F, t6521: F, t342: F, t4245: F, t450: F, t6032: F, t4314: F, t6025: F, t6509: F, t1107: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20892 = t1143 * t1148;
    let t20893 = t20891 * t20892;
    let t20896 = t19144 * t1561;
    let t20897 = t20896 * t4303;
    let t20900 = t19150 * t6521;
    let t20903 = t4245 * t342 * t450;
    let t20904 = t6032 * t20903;
    let t20906 = t20891 * t4314;
    let t20910 = t6025 * t6509 * t1148;
    let t20913 = t1107 * t6509;
    (t20892, t20893, t20896, t20897, t20900, t20903, t20904, t20906, t20910, t20913)
}
