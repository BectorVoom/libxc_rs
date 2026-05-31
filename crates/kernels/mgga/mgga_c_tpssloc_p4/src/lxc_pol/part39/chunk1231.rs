//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1231/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1231<F: Float>(t1227: F, t15643: F, t11705: F, t11719: F, t11728: F, t11734: F, t11746: F, t15610: F, t15612: F, t15617: F, t15622: F, t15627: F, t15631: F, t15637: F, t15642: F, t3490: F, t3496: F, t3506: F, t3515: F, t4974: F, t4984: F, t5019: F) -> F {
    let t15645 = t1227 * t15643 / F::cast_from(1728.0_f64);
    let t15648 = -t11705 / F::cast_from(3456.0_f64) - t5019 * t3496 / F::cast_from(576.0_f64) + t11746 / F::cast_from(2304.0_f64) - t15610 - t1227 * t15612 / F::cast_from(2304.0_f64) - t1227 * t15617 / F::cast_from(768.0_f64) + t3506 * t15622 / F::cast_from(1536.0_f64) + t11719 * t15627 / F::cast_from(512.0_f64) - t11728 * t15631 / F::cast_from(512.0_f64) - t11734 * t4984 / F::cast_from(1536.0_f64) - t3515 * t15637 / F::cast_from(1536.0_f64) + t15642 - t15645 - t3490 * t4974 / F::cast_from(1152.0_f64);
    t15648
}
