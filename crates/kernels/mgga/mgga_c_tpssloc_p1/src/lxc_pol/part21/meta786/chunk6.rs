//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2732/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2732<F: Float>(t1352: F, t5286: F, t3787: F, t6434: F, t1338: F, t20009: F, t1336: F, t1381: F, t16133: F, t16206: F, t16414: F, t1814: F, t1838: F, t19657: F, t19815: F, t3793: F, t3851: F, t3898: F, t3902: F, t5230: F, t5234: F, t5335: F, t5344: F, t5348: F, t5351: F, t53909: F, t544: F, t553: F, t56923: F, t57485: F) -> F {
    let t57643 = t1352 * t5286;
    let t57653 = t3787 * t6434;
    let t57659 = t1338 * t20009;
    let t57667 = -F::new(2.0) * t1336 * t1352 * t57659 - F::new(2.0) * t1336 * t16206 * t5348 - t1336 * t19657 * t3851 + F::new(2.0) * t1336 * t3793 * t57653 - F::new(4.0) * t5335 * t5344 * t57643 + t544 * t553 * t57485 - F::new(2.0) * t1381 * t56923 - F::new(4.0) * t16133 * t5234 + F::new(2.0) * t16414 * t1814 - F::new(2.0) * t1838 * t53909 + F::new(2.0) * t19815 * t3898 - F::new(2.0) * t19815 * t3902 + F::new(4.0) * t5230 * t5351;
    t57667
}
