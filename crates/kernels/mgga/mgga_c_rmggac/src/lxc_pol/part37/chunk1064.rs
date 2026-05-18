//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1064/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1064<F: Float>(t15881: F, t874: F, t352: F, t15888: F, t275: F, t1356: F, t74800: F, t74824: F, t74830: F, t74831: F, t74842: F, t74846: F, t77222: F, t77224: F, t77225: F, t77228: F, t77229: F, t77230: F, t77231: F, t77236: F, t77237: F) -> (F, F) {
    let t80162 = t874 * t15881;
    let t80163 = t80162 * t352;
    let t80167 = t275 * t15888;
    let t80170 = -t77222 + F::new(0.39914139006212695214e-1) * t1356 * t80163 + t74800 + t77224 + t77225 + t77228 + t77229 + t77230 + t77231 - t74824 + t74830 - F::new(0.58171619854173713844e-5) * t74831 + t80167 - t77236 + t77237 + F::new(0.17519306092901367186e-5) * t74842 + F::new(0.35038612185802734374e-6) * t74846;
    (t80163, t80170)
}
