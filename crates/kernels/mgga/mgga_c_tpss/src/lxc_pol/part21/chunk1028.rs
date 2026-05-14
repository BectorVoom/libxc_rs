//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1028/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1028<F: Float>(t11259: F, t2482: F, t8710: F, t1425: F, t2531: F, t2633: F, t3894: F, t2629: F, t3904: F, t11004: F, t10982: F, t10980: F, t10986: F, t11002: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8927: F) -> (F, F, F, F, F) {
    let t11260 = t11259 * t2482;
    let t11262 = 0.51726012919273400301e3 * t8710 * t11260;
    let t11263 = t1425 * t2482;
    let t11265 = 6.0 * t2531 * t11263;
    let t11267 = 0.11696447245269292414e1 * t3894 * t2633;
    let t11269 = 0.11696447245269292414e1 * t2629 * t3904;
    let t11276 = 0.2283111111111111111e-1 * t11004;
    let t11277 = 0.11415555555555555555e-1 * t10982;
    let t11286 = -t8927 - 0.1522074074074074074e-1 * t8616 + 0.38051851851851851851e-2 * t8607 - 0.11415555555555555555e-1 * t8618 + 0.57077777777777777777e-2 * t8605 - 0.76103703703703703702e-2 * t10980 + 0.76103703703703703701e-2 * t11002 - t11276 + t11277 - 0.19025925925925925925e-1 * t11010 + 0.68493333333333333331e-1 * t11015 - 0.2283111111111111111e-1 * t11020 - 0.11415555555555555555e-1 * t11024 - 0.10274e0 * t11028 + 0.68493333333333333332e-1 * t11033 + 0.34246666666666666666e-1 * t11037 - 0.17123333333333333333e-1 * t10986;
    (t11262, t11265, t11267, t11269, t11286)
}
