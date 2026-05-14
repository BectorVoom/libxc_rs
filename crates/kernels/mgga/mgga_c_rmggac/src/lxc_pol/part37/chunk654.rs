//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 654/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk654<F: Float>(t71418: F, t1341: F, t638: F, t703: F, t7310: F, t69760: F, t69832: F, t69934: F, t69936: F, t69938: F, t69940: F, t69942: F, t14696: F, t7335: F, t2019: F, t3180: F, t7926: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t71419 = 0.30487649791575028314e-3 * t71418;
    let t71446 = t638 * t7310 * t703 * t1341;
    let t71447 = 0.30487649791575028314e-3 * t71446;
    let t71486 = 0.10986805899793472145e-3 * t69760;
    let t71505 = 0.68400385060046895e-6 * t69832;
    let t71544 = 0.30487649791575028312e-3 * t69934;
    let t71545 = 0.32526727992809621482e-4 * t69936;
    let t71546 = 0.60975299583150056624e-3 * t69938;
    let t71551 = 0.16263363996404810741e-4 * t69940;
    let t71552 = 0.16263363996404810741e-4 * t69942;
    let t71564 = t7335 * t14696;
    let t71565 = 0.15243824895787514157e-3 * t71564;
    let t71581 = t2019 * t7926 * t3180;
    (t71419, t71447, t71486, t71505, t71544, t71545, t71546, t71551, t71552, t71565, t71581)
}
