//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1046/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1046<F: Float>(t4923: F, t8749: F, t8752: F, t903: F, t912: F, t3894: F, t3900: F, t2629: F, t4957: F, t3909: F, t3883: F, t3899: F, t10980: F, t11002: F, t11276: F, t11277: F, t14459: F, t14492: F, t14495: F, t14505: F, t14507: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t8616: F, t8927: F) -> (F, F, F, F, F, F) {
    let t14690 = t8749 * t4923;
    let t14691 = t8752 * t903;
    let t14692 = t14690 * t14691;
    let t14694 = 0.10254018858216406658e4 * t912 * t14692;
    let t14696 = 0.23392894490538584828e1 * t3894 * t3900;
    let t14698 = 0.5848223622634646207e0 * t2629 * t4957;
    let t14700 = 0.34631718211362927517e2 * t3894 * t3909;
    let t14701 = t3899 * t3883;
    let t14703 = 0.23392894490538584828e1 * t912 * t14701;
    let t14719 = -t8927 - 0.76103703703703703703e-2 * t8616 - 0.1522074074074074074e-1 * t10980 + 0.761037037037037037e-2 * t11002 - t11276 + t11277 + 0.3805185185185185185e-2 * t14495 - 0.19025925925925925925e-1 * t14517 + 0.68493333333333333331e-1 * t14459 - 0.2283111111111111111e-1 * t14521 - 0.11415555555555555555e-1 * t14505 - 0.10274e0 * t14525 + 0.68493333333333333332e-1 * t14528 + 0.57077777777777777777e-2 * t14507 - 0.11415555555555555555e-1 * t14532 + 0.34246666666666666666e-1 * t14535 - 0.17123333333333333333e-1 * t14492;
    (t14694, t14696, t14698, t14700, t14703, t14719)
}
