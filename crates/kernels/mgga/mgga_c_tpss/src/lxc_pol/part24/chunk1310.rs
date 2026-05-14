//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1310/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1310<F: Float>(t14229: F, t17964: F, t14234: F, t14176: F, t19703: F, t4708: F, t61072: F, t17946: F, t4712: F, t14207: F, t61051: F, t61063: F, t63961: F, t66410: F, t66418: F, t66422: F, t69972: F) -> (F,) {
    let t69974 = t17964 * t14229;
    let t69976 = t17964 * t14234;
    let t69978 = t19703 * t14176;
    let t69981 = t61072 * t4708;
    let t69983 = t17946 * t4712;
    let t69985 = t17964 * t14207;
    let t69988 = -t69972 / 48.0 + t69974 / 192.0 + t69976 / 192.0 - t69978 / 192.0 - 119.0 / 6912.0 * t61051 - 7.0 / 48.0 * t69981 + 7.0 / 144.0 * t69983 + t69985 / 384.0 - t66410 - 35.0 / 216.0 * t61063 - t66418 + t63961 - t66422;
    (t69988,)
}
