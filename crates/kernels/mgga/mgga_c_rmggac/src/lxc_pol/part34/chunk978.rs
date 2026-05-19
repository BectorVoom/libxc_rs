//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 978/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk978<F: Float>(t77217: F, t16503: F, t35039: F, t699: F, t9169: F, t30221: F, t3194: F, t74803: F, t14703: F, t289: F, t623: F, t71151: F, t71162: F, t74779: F, t74782: F, t74786: F, t74800: F, t77206: F, t77208: F, t77209: F, t77212: F, t77213: F, t77214: F) -> F {
    let t77218 = F::cast_from(0.53205749866622299248e-5_f64) * t77217;
    let t77221 = t16503 * t35039 * t699 * t9169;
    let t77222 = F::cast_from(0.42564599893297839398e-5_f64) * t77221;
    let t77224 = F::cast_from(0.39914139006212695214e-1_f64) * t30221 * t3194;
    let t77225 = F::cast_from(0.2727466165424534173e-1_f64) * t74803;
    let t77226 = F::cast_from(0.6505345598561924296e-5_f64) * t74779 - t74782 - F::cast_from(0.19957069503106347607e-1_f64) * t623 * t14703 + t71151 - t77206 + F::cast_from(0.72714524817717142308e-5_f64) * t74786 - t77208 - F::new(0.2363e1) * t289 * t77209 - t77212 - t77213 + t71162 + t77214 - t77218 - t77222 + t74800 + t77224 + t77225;
    t77226
}
