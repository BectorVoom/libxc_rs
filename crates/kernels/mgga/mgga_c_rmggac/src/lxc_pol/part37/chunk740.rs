//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 740/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk740<F: Float>(t2227: F, t235: F, t7262: F, t14696: F, t7491: F, t1341: F, t638: F, t703: F, t7310: F, t69760: F, t69832: F, t69934: F) -> (F, F, F, F, F, F) {
    let t71404 = t235 * t7262 * t2227;
    let t71418 = t7491 * t14696;
    let t71419 = F::cast_from(0.30487649791575028314e-3_f64) * t71418;
    let t71446 = t638 * t7310 * t703 * t1341;
    let t71447 = F::cast_from(0.30487649791575028314e-3_f64) * t71446;
    let t71486 = F::cast_from(0.10986805899793472145e-3_f64) * t69760;
    let t71505 = F::cast_from(0.68400385060046895e-6_f64) * t69832;
    let t71544 = F::cast_from(0.30487649791575028312e-3_f64) * t69934;
    (t71404, t71419, t71447, t71486, t71505, t71544)
}
