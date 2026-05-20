//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2087/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2087<F: Float>(t10041: F, t2563: F, t2678: F, t776: F, t222: F, t39934: F, t2617: F, t9637: F, t2649: F, t2691: F, t812: F, t815: F) -> (F, F, F, F, F, F) {
    let t41088 = t2563 * t10041;
    let t41090 = t776 * t2678;
    let t41096 = F::new(455.0) / F::new(243.0) * t39934 * t222;
    let t41107 = t2617 * t9637;
    let t41108 = t41107 * t2649;
    let t41115 = t812 * t815 * t2691;
    (t41088, t41090, t41096, t41107, t41108, t41115)
}
