//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1239/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1239<F: Float>(t23384: F, t23715: F, t210: F, t23632: F, t23668: F, t225: F, t82390: F, t23518: F, t6733: F, t23628: F, t6680: F, t10305: F, t10316: F, t10321: F, t23323: F, t23346: F, t23601: F, t23605: F, t23637: F, t23657: F, t23673: F, t23685: F, t23687: F, t25713: F, t2780: F, t3016: F, t6687: F, t6784: F, t6785: F, t6787: F, t6797: F, t6805: F, t6806: F, t82382: F, t82385: F) -> (F,) {
    let t82661 = t23384 * t23715;
    let t82668 = t23668 * t210 * t23632;
    let t82676 = t82390 * t225;
    let t82683 = t6733 * t23518;
    let t82694 = t6680 * t23628;
    let t82705 = -0.54831135561607547883e-2 * t82661 + 0.27415567780803773942e-2 * t6687 * t6784 * t6785 * t10321 - 0.43864908449286038307e-1 * t82668 * t23637 + 0.24125699647107321069e0 * t23323 * t6806 - 0.24674011002723396548e-1 * t6797 * t23657 * t23673 + 0.8529287754027840782e-2 * t6687 * t82676 * t6785 * t10305 + 0.80418998823691070229e-1 * t82382 * t6787 + 0.24674011002723396548e-1 * t23601 * t82683 * t23605 - 0.24674011002723396548e-1 * t6687 * t82385 * t25713 + 0.82246703342411321826e-2 * t6687 * t6784 * t23685 * t2780 - 0.43864908449286038307e-1 * t82694 + 0.16449340668482264365e-1 * t6687 * t6784 * t6785 * t10316 - 0.24674011002723396548e-1 * t6687 * t3016 * t6805 - 0.43864908449286038307e-1 * t23346 * t23687;
    (t82705,)
}
