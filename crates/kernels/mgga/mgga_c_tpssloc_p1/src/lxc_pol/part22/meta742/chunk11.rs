//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2463/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2463<F: Float>(t225: F, t69840: F, t1049: F, t21390: F, t1058: F, t1060: F, t11034: F, t11059: F, t11060: F, t11065: F, t14608: F, t1632: F, t17876: F, t18103: F, t18117: F, t18131: F, t21617: F, t21643: F, t21644: F, t21647: F, t21650: F, t3186: F, t3188: F, t3200: F, t43470: F, t43473: F, t4649: F, t4684: F, t5914: F, t69996: F) -> (F, F, F) {
    let t70012 = t69840 * t225;
    let t70014 = t1049 * t21390;
    let t70068 = F::new(3.0) * t1058 * t1060 * t4649 * t5914 + F::new(6.0) * t11059 * t11060 * t70014 - F::new(18.0) * t11065 * t18103 * t21643 - F::new(3.0) * t21617 * t3200 * t4684 + F::new(6.0) * t3186 * t3188 * t69996 + F::new(6.0) * t11034 * t21644 - F::new(3.0) * t14608 * t18117 - F::new(6.0) * t14608 * t18131 + F::new(3.0) * t1632 * t17876 + F::new(6.0) * t21647 * t43473 - F::new(6.0) * t21650 * t43470;
    (t70012, t70014, t70068)
}
