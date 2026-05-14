//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1352/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1352<F: Float>(t25: F, t28: F, t54312: F, t54314: F, t54316: F, t6305: F, t5397: F, t19547: F, t20216: F, t3664: F, t39419: F, t5134: F, t514: F, t75911: F, t6312: F, t5966: F, t19559: F, t20390: F, t3672: F, t39436: F, t5142: F, t517: F, t77953: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t79856 = 96.0 * t54312;
    let t79857 = 576.0 * t54314;
    let t79858 = 384.0 * t54316;
    let t79859 = t6305 * t6305;
    let t79864 = t5397 * t5397;
    let t79872 = piecewise3(t26, 0.0, 40.0 / 81.0 * t39419 * t79859 - 16.0 / 9.0 * t19547 * t5397 + 4.0 / 3.0 * t3664 * t79864 + 16.0 / 9.0 * t5134 * t20216 + 4.0 / 3.0 * t514 * t75911);
    let t79873 = t6312 * t6312;
    let t79878 = t5966 * t5966;
    let t79886 = piecewise3(t29, 0.0, 40.0 / 81.0 * t39436 * t79873 - 16.0 / 9.0 * t19559 * t5966 + 4.0 / 3.0 * t3672 * t79878 + 16.0 / 9.0 * t5142 * t20390 + 4.0 / 3.0 * t517 * t77953);
    (t79856, t79857, t79858, t79859, t79864, t79872, t79873, t79878, t79886)
}
