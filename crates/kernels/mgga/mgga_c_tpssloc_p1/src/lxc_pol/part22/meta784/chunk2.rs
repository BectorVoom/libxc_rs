//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2693/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2693<F: Float>(t12171: F, t1336: F, t1352: F, t16060: F, t16132: F, t1840: F, t19658: F, t19660: F, t19752: F, t19805: F, t19815: F, t20495: F, t20648: F, t26322: F, t3777: F, t5234: F, t5339: F, t5341: F, t5344: F, t6420: F, t6454: F, t74967: F) -> F {
    let t75101 = F::cast_from(6.0_f64) * t12171 * t1336 * t20495 - F::cast_from(3.0_f64) * t1336 * t16132 * t6420 - t1352 * t5344 * t74967 - F::cast_from(3.0_f64) * t19660 * t26322 * t5344 - F::cast_from(3.0_f64) * t16060 * t6454 + F::cast_from(3.0_f64) * t1840 * t19805 - F::cast_from(3.0_f64) * t19658 * t5234 - F::cast_from(6.0_f64) * t19752 * t5234 - F::cast_from(3.0_f64) * t19815 * t5339 - F::cast_from(3.0_f64) * t19815 * t5341 - F::cast_from(3.0_f64) * t20648 * t3777;
    t75101
}
