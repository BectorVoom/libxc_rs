//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1936/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1936<F: Float>(t1227: F, t15643: F, t11705: F, t11719: F, t11728: F, t11734: F, t11746: F, t15610: F, t15612: F, t15617: F, t15622: F, t15627: F, t15631: F, t15637: F, t15642: F, t3490: F, t3496: F, t3506: F, t3515: F, t4974: F, t4984: F, t5019: F) -> F {
    let t15645 = t1227 * t15643 / F::new(1728.0);
    let t15648 = -t11705 / F::new(3456.0) - t5019 * t3496 / F::new(576.0) + t11746 / F::new(2304.0) - t15610 - t1227 * t15612 / F::new(2304.0) - t1227 * t15617 / F::new(768.0) + t3506 * t15622 / F::new(1536.0) + t11719 * t15627 / F::new(512.0) - t11728 * t15631 / F::new(512.0) - t11734 * t4984 / F::new(1536.0) - t3515 * t15637 / F::new(1536.0) + t15642 - t15645 - t3490 * t4974 / F::new(1152.0);
    t15648
}
