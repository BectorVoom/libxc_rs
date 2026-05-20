//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1296/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296<F: Float>(t185: F, t2658: F, t75847: F, t57897: F, t1484: F, t16606: F, t2522: F, t39249: F, t39256: F, t39309: F, t39312: F, t4314: F, t5527: F, t67239: F, t75839: F, t75840: F, t75844: F, t75845: F, t75846: F) -> (F, F, F) {
    let t75850 = F::new(36.0) * t2658 * t185 * t75847;
    let t75851 = F::new(6.0) * t57897;
    let t75852 = F::new(24.0) * t1484 * t2522 * t67239 + F::new(36.0) * t16606 * t4314 * t5527 - t39249 - t39256 - t39309 + t39312 + t75839 - t75840 - t75844 - t75845 + t75846 + t75850 + t75851;
    (t75850, t75851, t75852)
}
