//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1065/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1065<F: Float>(t14981: F, t5928: F, t74850: F, t74856: F, t74870: F, t74889: F, t77242: F, t77243: F, t77244: F, t77246: F, t77247: F, t77249: F, t77250: F, t77251: F, t77252: F, t77253: F, t77254: F, t77255: F) -> F {
    let t80176 = -F::new(0.35038612185802734374e-6) * t74850 + F::new(0.87596530464506835936e-6) * t74856 + t77242 - t77243 + t77244 - t77246 + t74870 + F::new(0.39914139006212695214e-1) * t5928 * t14981 + t77247 - F::new(0.58171619854173713844e-5) * t74889 + t77249 + t77250 + t77251 - t77252 - t77253 - t77254 + t77255;
    t80176
}
