//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1807/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1807<F: Float>(t82011: F, t82039: F, t25273: F, t6579: F, t244: F, t268: F, t6559: F, t25250: F, t87202: F, t25316: F, t82038: F, t23110: F, t23185: F, t25272: F) -> (F, F, F, F, F, F, F) {
    let t87687 = F::cast_from(0.12793931631041761173e0_f64) * t82011;
    let t87708 = F::cast_from(0.10417915756705434098e0_f64) * t82039;
    let t87709 = t6579 * t25273;
    let t87712 = t6559 * t244 * t268;
    let t87714 = t87712 * t87202 * t25250;
    let t87718 = t82038 * t25316;
    let t87729 = t23185 * t23110 * t25272;
    (t87687, t87708, t87709, t87712, t87714, t87718, t87729)
}
