//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 749/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk749<F: Float>(t14116: F, t21709: F, t9152: F, t14225: F, t7248: F, t8421: F, t8426: F, t9188: F, t3352: F, t8431: F, t3157: F, t33235: F, t15310: F, t52781: F, t10570: F, t1652: F, t262: F, t3068: F) -> (F, F, F, F, F, F, F) {
    let t75626 = t14116 * t21709 * t9152;
    let t75629 = t14225 * t7248 * t8421;
    let t75632 = t14225 * t9188 * t8426;
    let t75635 = t14225 * t3352 * t8431;
    let t75638 = t33235 * t3157;
    let t75640 = t52781 * t15310;
    let t75644 = t10570 * t3068 * t262 * t1652;
    (t75626, t75629, t75632, t75635, t75638, t75640, t75644)
}
