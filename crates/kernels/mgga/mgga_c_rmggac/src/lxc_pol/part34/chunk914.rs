//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 914/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk914<F: Float>(t15386: F, t15652: F, t15653: F, t15654: F, t15655: F, t15656: F, t15657: F, t15658: F, t15660: F, t15661: F, t15662: F, t15663: F, t15664: F, t15665: F, t70745: F, t14275: F, t14702: F, t14705: F, t14709: F, t14712: F, t14913: F, t14918: F, t14919: F, t15666: F, t15667: F, t15668: F, t15671: F, t15674: F, t15677: F, t70746: F, t72207: F) -> (F, F) {
    let t78629 = t15652 + t15653 - t15654 + t15655 + t15656 - t70745 - t15657 + t15658 - t15660 - t15661 + t15662 + t15386 - t15663 + t15664 + t15665;
    let t78630 = -t15666 + t14913 + t70746 + t15667 - t15668 + t15671 - t15674 - t15677 + t14702 + t14275 - t14705 + t14918 + t14919 + t72207 - t14709 - t14712;
    (t78629, t78630)
}
