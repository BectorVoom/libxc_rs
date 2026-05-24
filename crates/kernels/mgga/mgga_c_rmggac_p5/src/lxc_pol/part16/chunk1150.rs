//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1150/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1150<F: Float>(t10283: F, t10284: F, t42416: F, t42417: F, t42418: F, t42420: F, t42421: F, t42424: F, t48139: F, t48157: F, t48176: F, t48193: F, t48212: F, t48225: F, t48237: F, t48274: F, t48297: F, t48307: F, t48324: F, t48342: F, t48357: F, t48376: F, t48394: F, t48407: F, t48429: F, t48450: F, t48469: F, t48498: F, t48520: F, t48545: F, t48564: F, t48587: F, t48609: F, t48626: F, t48641: F, t48662: F, t48684: F, t48706: F, t48727: F, t48742: F, t48763: F, t48777: F, t48795: F, t48818: F, t48838: F, t48849: F, t48864: F, t48877: F, t48901: F, t48924: F, t48946: F, t48967: F, t48990: F, t49006: F, t49032: F, t49175: F, t49199: F, t49220: F, t49237: F, t49256: F, t49277: F, t49626: F, t49649: F, t49666: F, t49686: F, t49709: F, t49725: F, t49747: F, t49770: F, t49787: F, t49803: F, t49818: F, t8: F, t8081: F, t8520: F) -> F {
    let t49827 = t42416 + t42417 + t10283 + t8 * (t49818 + t49803 + t49787 + t49770 + t49747 + t49725 + t49709 + t49686 + t49666 + t49649 + t49626 + t49277 + t49256 + t49237 + t49220 + t49199 + t49175 + t49032 + t49006 + t48990 + t48967 + t48946 + t48924 + t48901 + t48877 + t48864 + t48849 + t48838 + t48818 + t48795 + t48777 + t48763 + t48742 + t48727 + t48706 + t48684 + t48662 + t48641 + t48626 + t48609 + t48587 + t48564 + t48545 + t48520 + t48498 + t48469 + t48450 + t48429 + t48342 + t48157 + t48307 + t48324 + t48225 + t48176 + t48407 + t48212 + t48139 + t48193 + t48357 + t48237 + t48297 + t48376 + t48274 + t48394) + t42418 + F::cast_from(0.47885174879960069325e-4_f64) * t8520 - t10284 - t42420 - t42421 + t8081 - t42424;
    t49827
}
