//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1013/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1013<F: Float>(t1174: F, t11754: F, t11692: F, t11694: F, t11699: F, t11703: F, t11705: F, t11709: F, t11719: F, t11724: F, t11728: F, t11731: F, t11734: F, t11738: F, t11741: F, t11746: F, t11748: F, t11751: F, t3511: F, t3518: F) -> F {
    let t11755 = t1174 * t11754;
    let t11757 = t11692 * t11694 / F::new(1536.0) - t11699 / F::new(1152.0) + t11703 / F::new(1536.0) - t11705 / F::new(1152.0) + t11709 * t3511 / F::new(512.0) + t11719 * t11724 / F::new(512.0) - t11728 * t11731 / F::new(512.0) - t11734 * t3518 / F::new(1024.0) + t11738 * t11741 / F::new(3072.0) + t11746 / F::new(768.0) + t1174 * t11748 / F::new(72.0) - t1174 * t11751 / F::new(48.0) + t11755 / F::new(216.0);
    t11757
}
