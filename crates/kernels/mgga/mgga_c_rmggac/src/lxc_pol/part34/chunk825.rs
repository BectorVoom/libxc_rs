//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 825/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk825<F: Float>(t77221: F, t30221: F, t3194: F, t74803: F, t14703: F, t289: F, t623: F, t71151: F, t71162: F, t74779: F, t74782: F, t74786: F, t74800: F, t77206: F, t77208: F, t77209: F, t77212: F, t77213: F, t77214: F, t77218: F) -> (F,) {
    let t77222 = 0.42564599893297839398e-5 * t77221;
    let t77224 = 0.39914139006212695214e-1 * t30221 * t3194;
    let t77225 = 0.2727466165424534173e-1 * t74803;
    let t77226 = 0.6505345598561924296e-5 * t74779 - t74782 - 0.19957069503106347607e-1 * t623 * t14703 + t71151 - t77206 + 0.72714524817717142308e-5 * t74786 - t77208 - 0.2363e1 * t289 * t77209 - t77212 - t77213 + t71162 + t77214 - t77218 - t77222 + t74800 + t77224 + t77225;
    (t77226,)
}
