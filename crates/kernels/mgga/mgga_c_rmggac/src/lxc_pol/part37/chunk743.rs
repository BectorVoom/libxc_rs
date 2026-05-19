//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 743/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk743<F: Float>(t71632: F, t3219: F, t7939: F, t699: F, t830: F, t739: F, t1327: F, t640: F, t702: F, t7323: F, t70071: F, t70078: F) -> (F, F, F, F, F, F, F) {
    let t71633 = F::cast_from(0.33105799917009430643e-4_f64) * t71632;
    let t71634 = t7939 * t3219;
    let t71637 = t699 * t830;
    let t71638 = t739 * t71637;
    let t71639 = F::cast_from(0.14635184302277988245e0_f64) * t71638;
    let t71660 = t7323 * t640 * t702 * t1327;
    let t71661 = F::cast_from(0.34200192530023447503e-6_f64) * t71660;
    let t71670 = F::cast_from(0.66671395154821946452e-1_f64) * t70071;
    let t71671 = F::cast_from(0.39032073591371545778e-3_f64) * t70078;
    (t71633, t71634, t71637, t71639, t71661, t71670, t71671)
}
