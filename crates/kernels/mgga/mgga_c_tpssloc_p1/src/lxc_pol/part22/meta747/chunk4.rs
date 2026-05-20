//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2492/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2492<F: Float>(t21569: F, t3070: F, t42488: F, t10403: F, t10408: F, t17156: F, t18014: F, t3071: F, t4338: F, t4343: F, t48607: F, t50324: F, t5677: F, t5867: F, t5909: F, t62827: F, t62832: F, t62836: F, t62840: F, t69742: F, t70241: F) -> F {
    let t70912 = t3070 * t42488 * t21569;
    let t70917 = t50324 * t5909 / F::new(768.0) - t62827 / F::new(81.0) - t62832 / F::new(324.0) - t62836 / F::new(108.0) - F::new(5.0) / F::new(768.0) * t3070 * t10408 * t17156 * t70241 + t3070 * t3071 * t5677 * t70241 / F::new(256.0) - t3070 * t3071 * t5867 * t4343 / F::new(768.0) + t10403 * t3071 * t62840 * t18014 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10408 * t5867 * t4338 + F::new(5.0) / F::new(6912.0) * t70912 + t48607 * t3071 * t69742 / F::new(256.0);
    t70917
}
