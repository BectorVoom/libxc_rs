//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1097/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1097<F: Float>(t154: F, t21: F, t6896: F, t6898: F, t12279: F, t26309: F, t12371: F, t12404: F, t22833: F, t12413: F, t12422: F, t12409: F, t22797: F, t3770: F, t12313: F, t6916: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t80741 = t21 * t154;
    let t80742 = t80741 * t6896;
    let t80743 = t80742 * t6898;
    let t80744 = 0.16220877603642232915e0 * t80743;
    let t80749 = t26309 * t12279;
    let t80751 = t26309 * t12371;
    let t80753 = t22833 * t12404;
    let t80755 = t22833 * t12413;
    let t80757 = t22833 * t12422;
    let t80759 = t22833 * t12409;
    let t80761 = t22797 * t3770;
    let t80763 = t6916 * t12313;
    (t80741, t80742, t80744, t80749, t80751, t80753, t80755, t80757, t80759, t80761, t80763)
}
