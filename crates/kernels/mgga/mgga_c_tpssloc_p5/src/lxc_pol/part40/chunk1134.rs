//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1134/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1134<F: Float>(t1022: F, t11060: F, t5928: F, t4684: F, t5936: F, t4673: F, t1058: F, t1061: F, t11034: F, t11037: F, t11046: F, t11059: F, t11065: F, t14618: F, t14651: F, t1630: F, t18081: F, t18083: F, t18086: F, t18089: F, t18094: F, t18100: F, t18104: F, t18108: F, t3180: F, t3186: F, t3200: F, t4674: F, t5929: F, t5937: F, t5939: F) -> F {
    let t18111 = t11060 * t1022;
    let t18112 = t5928 * t18111;
    let t18117 = t5936 * t4684;
    let t18121 = t5936 * t4673;
    let t18124 = F::new(2.0) * t1058 * t18089 + t1058 * t18100 + t1061 * t18086 + F::new(2.0) * t11034 * t5929 - t11037 * t5939 + t11046 * t18094 + F::new(6.0) * t11059 * t18112 - F::new(6.0) * t11065 * t18104 + F::new(4.0) * t14618 * t4674 + F::new(2.0) * t14651 * t1630 - t18081 * t3200 + F::new(2.0) * t18083 * t3186 - F::new(2.0) * t18108 * t3200 - t18117 * t3200 + F::new(2.0) * t18121 * t3186 + t3180 * t5937;
    t18124
}
