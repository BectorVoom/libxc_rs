//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1003/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1003<F: Float>(t1060: F, t14644: F, t14626: F, t3188: F, t1057: F, t14205: F, t11054: F, t1629: F, t1003: F, t1058: F, t1061: F, t1063: F, t11037: F, t11046: F, t13940: F, t14615: F, t14618: F, t14623: F, t14627: F, t14631: F, t14640: F, t1610: F, t3180: F, t3186: F, t3189: F, t3197: F, t3200: F, t3204: F, t353: F, t384: F, t4615: F, t4669: F, t4685: F, t4689: F, t4691: F) -> (F,) {
    let t14645 = t14644 * t1060;
    let t14648 = t14626 * t3188;
    let t14651 = t14205 * t1057;
    let t14654 = t1629 * t11054;
    let t14657 = 2.0 * t1003 * t4691 + 2.0 * t1058 * t14645 + 2.0 * t1061 * t14651 + 2.0 * t1063 * t4615 - 2.0 * t11037 * t4685 + t11046 * t14631 + t13940 * t384 - 2.0 * t14615 * t3200 + 2.0 * t14618 * t3189 - t14623 * t3200 - t14627 * t3200 + t14640 * t353 + 2.0 * t14648 * t3186 + 2.0 * t14654 * t3186 + t1610 * t3204 + 2.0 * t3180 * t4689 + t3197 * t4669;
    (t14657,)
}
