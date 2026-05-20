//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1038/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1038<F: Float>(t25826: F, t6704: F, t14555: F, t1635: F, t1956: F, t23327: F, t23369: F, t23392: F, t23579: F, t25798: F, t25802: F, t25807: F, t25811: F, t25816: F, t25820: F, t25822: F, t25824: F, t3169: F, t388: F, t4557: F, t6680: F, t6687: F, t6816: F, t7562: F, t7625: F) -> F {
    let t25827 = t6704 * t25826;
    let t25834 = F::cast_from(0.27415567780803773942e-2_f64) * t23392 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25798 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t25802 - t23369 * t1635 + F::cast_from(0.27415567780803773942e-2_f64) * t25807 + F::cast_from(0.91385225936012579807e-3_f64) * t23579 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t25811 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t25816 - t3169 * t7625 + t25820 * t388 + t25822 * t388 - F::cast_from(0.27415567780803773942e-2_f64) * t25824 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25827 - t4557 * t6816 - t14555 * t1956 - F::cast_from(0.21932454224643019153e-1_f64) * t6680 * t7562;
    t25834
}
