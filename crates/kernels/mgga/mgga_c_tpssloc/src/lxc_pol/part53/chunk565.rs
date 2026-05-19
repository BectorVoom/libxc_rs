//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 565/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk565<F: Float>(t3692: F, t1307: F, t1388: F, t2408: F, t2417: F, t2423: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t3918: F, t5122: F, t5126: F, t5127: F, t5131: F, t5153: F, t5156: F, t5159: F, t5160: F, t5161: F) -> (F, F) {
    let t5164 = F::cast_from(0.5848223622634646207e0_f64) * t3692;
    let t5165 = F::new(3.0) * t1307 * t3918 * t5122 + F::new(6.0) * t1307 * t5126 * t5127 - t1388 * t5160 * t5161 + F::new(3.0) * t3918 * t5131 + t2408 + t2417 - t2423 + t3686 + t3688 - t3690 - t3695 + t3813 + t5153 - t5156 - t5159 - t5164;
    (t5164, t5165)
}
