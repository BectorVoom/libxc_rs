//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1039/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1039<F: Float>(t1266: F, t2165: F, t2167: F, t2314: F, t26006: F, t26141: F, t26144: F, t26145: F, t26147: F, t26150: F, t26153: F, t26157: F, t4026: F, t4028: F, t4034: F, t5361: F, t7271: F, t7983: F, t7989: F) -> (F,) {
    let t27878 = -t1266 * t7983 - t2165 * t4026 + t2167 * t5361 - 2.0 * t2314 * t7989 - 2.0 * t4028 * t7271 - 2.0 * t4034 * t7989 - t26006 - t26141 - t26144 - t26145 + t26147 - t26150 + t26153 + t26157;
    (t27878,)
}
