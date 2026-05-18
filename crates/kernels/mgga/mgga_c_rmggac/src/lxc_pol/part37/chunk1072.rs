//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1072/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1072<F: Float>(t75074: F, t75062: F, t75065: F, t75069: F, t75072: F, t75081: F, t75092: F, t75100: F, t75103: F, t75106: F, t75108: F, t75110: F, t77445: F, t77447: F, t77450: F, t77452: F, t77458: F) -> F {
    let t80214 = F::new(0.65053455985619242964e-5) * t75074;
    let t80221 = -F::new(0.40878380883436523435e-5) * t75062 + F::new(0.40878380883436523435e-5) * t75065 + t75069 - t75072 + t80214 + t77445 - F::new(0.31062809106223861414e-2) * t75081 - t77447 - t77450 - F::new(0.87596530464506835936e-6) * t75092 + t77452 - F::new(0.58171619854173713844e-5) * t75100 - F::new(0.72714524817717142305e-5) * t75103 - F::new(0.10511583655740820312e-5) * t75106 - F::new(0.58171619854173713844e-5) * t75108 - t77458 + t75110;
    t80221
}
