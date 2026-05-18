//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 998/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk998<F: Float>(t77470: F, t2010: F, t2415: F, t8188: F, t14434: F, t5898: F, t75092: F, t75100: F, t75103: F, t75106: F, t75108: F, t75110: F, t75115: F, t77450: F, t77452: F, t77458: F, t77463: F, t77464: F, t77465: F, t77468: F, t884: F) -> F {
    let t77471 = F::new(0.36021158228745895953e-3) * t77470;
    let t77473 = t2010 * t2415 * t8188;
    let t77474 = F::new(0.36021158228745895953e-3) * t77473;
    let t77475 = -t77450 - F::new(0.8759653046450683594e-6) * t75092 + t77452 - F::new(0.58171619854173713846e-5) * t75100 - F::new(0.72714524817717142308e-5) * t75103 - F::new(0.10511583655740820313e-5) * t75106 - F::new(0.58171619854173713846e-5) * t75108 - t77458 + t75110 + t75115 - F::new(0.11974241701863808564e0) * t884 * t14434 * t5898 + t77463 - t77464 + t77465 - t77468 - t77471 - t77474;
    t77475
}
