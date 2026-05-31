//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1091/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1091<F: Float>(t1297: F, t1390: F, t193: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5167: F, t5169: F, t5187: F, t5263: F, t5265: F, t5267: F, t5268: F, t5269: F, t533: F, t5356: F) -> F {
    let t5360 = t1390 * t193 * t533 * t5356 + F::cast_from(3.0_f64) * t1297 * t193 * t5187 - t2426 - t2486 + t3819 - t3821 + t3825 + t3827 - t3832 + t5167 + t5169 - t5263 + t5265 - t5267 - t5268 - t5269;
    t5360
}
