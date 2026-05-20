//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1937/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1937<F: Float>(t1066: F, t1920: F, t23346: F, t23385: F, t23387: F, t23389: F, t25767: F, t25778: F, t25785: F, t25789: F, t25791: F, t3026: F, t3169: F, t388: F, t4557: F, t4660: F, t4665: F, t6687: F, t6771: F, t6776: F, t6816: F, t7554: F, t7566: F, t7600: F, t7625: F) -> F {
    let t25794 = -t4660 * t6816 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t25767 + F::new(2.0) * t6771 * t4665 + F::new(2.0) * t3169 * t7600 + F::cast_from(0.21932454224643019153e-1_f64) * t23346 * t7566 + F::new(2.0) * t4557 * t6776 - t25778 * t1066 - F::cast_from(0.27415567780803773942e-2_f64) * t23385 - F::cast_from(0.27415567780803773942e-2_f64) * t23387 - F::cast_from(0.73108180748810063845e-2_f64) * t23346 * t7554 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25785 - F::cast_from(0.73108180748810063845e-2_f64) * t23389 + t25789 * t388 + t25791 * t388 - t3026 * t7625;
    t25794
}
