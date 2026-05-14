//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 826/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk826<F: Float>(t74807: F, t74809: F, t74813: F, t74817: F, t15598: F, t333: F, t74835: F, t74839: F, t74858: F, t74861: F, t74864: F, t74824: F, t74830: F, t74831: F, t74842: F, t74846: F, t74850: F, t74856: F, t884: F) -> (F, F) {
    let t77228 = 0.2727466165424534173e-1 * t74807;
    let t77229 = 0.13637330827122670865e-1 * t74809;
    let t77230 = 0.13637330827122670865e-1 * t74813;
    let t77231 = 0.13637330827122670865e-1 * t74817;
    let t77233 = t15598 * t333;
    let t77236 = 0.69805943825008456614e-4 * t74835;
    let t77237 = 0.11634323970834742769e-3 * t74839;
    let t77242 = 0.1276937996798935182e-4 * t74858;
    let t77243 = 0.1276937996798935182e-4 * t74861;
    let t77244 = 0.638468998399467591e-4 * t74864;
    let t77245 = t77228 + t77229 + t77230 + t77231 - t74824 + t74830 - 0.58171619854173713846e-5 * t74831 + 0.59871208509319042821e-1 * t884 * t77233 - t77236 + t77237 + 0.17519306092901367187e-5 * t74842 + 0.35038612185802734376e-6 * t74846 - 0.35038612185802734376e-6 * t74850 + 0.8759653046450683594e-6 * t74856 + t77242 - t77243 + t77244;
    (t77233, t77245)
}
