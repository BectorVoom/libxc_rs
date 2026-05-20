//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2315/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2315<F: Float>(t67262: F, t67280: F, t12895: F, t193: F, t20756: F, t2522: F, t39549: F, t39563: F, t4314: F, t5527: F, t67226: F, t67228: F, t67231: F, t67235: F, t67239: F, t67244: F, t766: F, t776: F, t868: F, t870: F) -> (F, F) {
    let t67282 = t67262 / F::new(2.0) + t67280 / F::new(2.0);
    let t67286 = F::new(6.0) * t193 * t20756 * t868 * t870 + F::new(18.0) * t12895 * t4314 * t5527 + F::new(3.0) * t193 * t67282 * t766 + F::new(6.0) * t2522 * t67239 * t776 + F::new(6.0) * t4314 * t67235 * t776 + t39549 + t39563 + t67226 + t67228 + t67231 + t67244;
    (t67282, t67286)
}
