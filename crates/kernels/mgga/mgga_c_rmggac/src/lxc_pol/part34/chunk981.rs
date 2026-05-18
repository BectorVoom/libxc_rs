//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 981/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk981<F: Float>(t77259: F, t71196: F, t71204: F, t74870: F, t74889: F, t74919: F, t77246: F, t77247: F, t77249: F, t77250: F, t77251: F, t77252: F, t77253: F, t77254: F, t77255: F, t77256: F, t77258: F) -> F {
    let t77260 = F::new(0.19863479950205658386e-4) * t77259;
    let t77261 = -t77246 + t74870 + t77247 - F::new(0.58171619854173713846e-5) * t74889 + t77249 + t77250 + t77251 - t77252 - t77253 - t77254 + t77255 - t77256 + t71196 + F::new(0.24527028530061914063e-5) * t74919 + t77258 + t71204 - t77260;
    t77261
}
