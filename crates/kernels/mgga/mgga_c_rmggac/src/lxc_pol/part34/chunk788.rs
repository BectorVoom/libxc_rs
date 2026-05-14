//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 788/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk788<F: Float>(t14628: F, t14630: F, t14634: F, t14636: F, t14638: F, t14641: F, t14644: F, t14865: F, t15495: F, t15499: F, t15501: F, t15506: F, t68336: F, t70714: F, t70720: F, t14645: F, t14646: F, t14649: F, t14651: F, t14653: F, t14883: F, t15240: F, t15243: F, t15246: F, t15510: F, t15511: F, t15512: F, t15513: F, t15514: F, t70721: F, t70722: F) -> (F, F) {
    let t76601 = -t14628 + t68336 + t15495 + t15499 - t70714 + t14630 - t14865 - t15501 + t14634 + t14636 + t15506 - t14638 - t14641 + t70720 + t14644;
    let t76602 = -t14645 + t14646 + t15240 - t15243 - t15246 - t70721 + t70722 + t14649 + t14651 + t14883 + t15510 - t15511 + t15512 + t15513 - t15514 + t14653;
    (t76601, t76602)
}
