//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 811/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk811<F: Float>(t74793: F, t74795: F, t74797: F, t34975: F, t34976: F, t699: F, t8455: F, t16503: F, t35039: F, t9169: F, t30221: F, t3194: F, t74803: F, t74807: F, t74809: F, t74813: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77212 = 0.68186654135613354325e-2 * t74793;
    let t77213 = 0.68186654135613354325e-2 * t74795;
    let t77214 = 0.12263514265030957031e-4 * t74797;
    let t77217 = t34975 * t34976 * t699 * t8455;
    let t77218 = 0.53205749866622299248e-5 * t77217;
    let t77221 = t16503 * t35039 * t699 * t9169;
    let t77222 = 0.42564599893297839398e-5 * t77221;
    let t77224 = 0.39914139006212695214e-1 * t30221 * t3194;
    let t77225 = 0.2727466165424534173e-1 * t74803;
    let t77228 = 0.2727466165424534173e-1 * t74807;
    let t77229 = 0.13637330827122670865e-1 * t74809;
    let t77230 = 0.13637330827122670865e-1 * t74813;
    (t77212, t77213, t77214, t77218, t77222, t77224, t77225, t77228, t77229, t77230)
}
