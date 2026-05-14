//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1296/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1296<F: Float>(t1119: F, t43954: F, t11180: F, t3308: F, t3256: F, t3312: F, t3316: F, t11270: F, t3259: F, t1094: F, t11274: F, t11278: F, t3262: F, t3311: F, t409: F, t3265: F) -> (F, F, F, F, F, F, F) {
    let t43956 = 4.0 * t43954 * t1119;
    let t43958 = 6.0 * t11180 * t3308;
    let t43959 = t3256 * t3312;
    let t43961 = 0.96491876992155210402e2 * t43959 * t3316;
    let t43963 = 4.0 * t3259 * t11270;
    let t43964 = t1094 * t11274;
    let t43966 = 0.2069040516770936012e4 * t43964 * t11278;
    let t43969 = t409 / t3311 / t3262;
    let t43970 = t3265 * t3265;
    (t43956, t43958, t43961, t43963, t43966, t43969, t43970)
}
