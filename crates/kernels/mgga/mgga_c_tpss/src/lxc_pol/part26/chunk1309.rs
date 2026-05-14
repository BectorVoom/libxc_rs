//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1309/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1309<F: Float>(t14223: F, t19703: F, t14256: F, t19696: F, t215: F, t63908: F, t63914: F, t63918: F, t63929: F, t69926: F, t69928: F, t69930: F, t69932: F, t69934: F, t69936: F, t14240: F, t63993: F) -> (F, F) {
    let t69938 = t19703 * t14223;
    let t69941 = t19696 * t215 * t14256;
    let t69943 = -5.0 / 192.0 * t69926 + t69928 / 192.0 - t69930 / 96.0 - t63908 + t69932 / 384.0 + t69934 / 384.0 - 7.0 / 288.0 * t69936 + t69938 / 768.0 - t63914 - t63918 + t63929 + t69941 / 16.0;
    let t69945 = t63993 * t215 * t14240;
    (t69943, t69945)
}
