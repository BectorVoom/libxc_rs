//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 713/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk713<F: Float>(t182: F, t5151: F, t172: F, t1787: F, t763: F, t67: F, t758: F, t193: F, t533: F, t1845: F, t3701: F, t3692: F, t1307: F, t1388: F, t2408: F, t2417: F, t2423: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t3918: F, t5122: F, t5126: F, t5127: F, t5131: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5153 = 0.19751673498613801407e-1 * t5151 * t182;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5156 = 0.5848223622634646207e0 * t5155;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5159 = 0.18311447306006545054e-3 * t5158;
    let t5160 = t193 * t533;
    let t5161 = t1845 * t3701;
    let t5164 = 0.5848223622634646207e0 * t3692;
    let t5165 = 3.0 * t1307 * t3918 * t5122 + 6.0 * t1307 * t5126 * t5127 - t1388 * t5160 * t5161 + 3.0 * t3918 * t5131 + t2408 + t2417 - t2423 + t3686 + t3688 - t3690 - t3695 + t3813 + t5153 - t5156 - t5159 - t5164;
    (t5153, t5154, t5155, t5156, t5157, t5158, t5159, t5160, t5161, t5164, t5165)
}
