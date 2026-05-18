//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1094/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1094<F: Float>(t15386: F, t15652: F, t15653: F, t15654: F, t15655: F, t15656: F, t15657: F, t15658: F, t15660: F, t15661: F, t15662: F, t15663: F, t15664: F, t15665: F, t70745: F) -> F {
    let t78629 = t15652 + t15653 - t15654 + t15655 + t15656 - t70745 - t15657 + t15658 - t15660 - t15661 + t15662 + t15386 - t15663 + t15664 + t15665;
    t78629
}
