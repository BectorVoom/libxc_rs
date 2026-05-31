//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1464/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1464<F: Float>(t120912: F, t120924: F, t120926: F, t120928: F, t120930: F, t120940: F, t120941: F, t123198: F, t2314: F, t26875: F, t26902: F, t31832: F, t32350: F, t34150: F, t4034: F, t4073: F, t652: F, t7408: F, t7801: F, t7941: F, t8690: F) -> F {
    let t124890 = -F::cast_from(2.0_f64) * t652 * t7408 * t7801 + F::cast_from(6.0_f64) * t123198 * t26875 - F::cast_from(2.0_f64) * t2314 * t34150 - t26902 * t8690 + t31832 * t7941 - F::cast_from(2.0_f64) * t32350 * t4073 - F::cast_from(2.0_f64) * t34150 * t4034 - t120912 - t120924 - t120926 - t120928 - t120930 + t120940 - t120941;
    t124890
}
