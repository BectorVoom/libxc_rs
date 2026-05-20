//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2733/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2733<F: Float>(t12259: F, t12267: F, t1332: F, t1336: F, t16127: F, t16132: F, t16423: F, t19657: F, t19658: F, t19813: F, t19815: F, t20010: F, t3773: F, t3777: F, t3856: F, t3905: F, t3907: F, t40486: F, t5234: F, t5287: F, t6388: F, t6415: F, t6456: F, t6458: F) -> F {
    let t57692 = -t12259 * t1336 * t6415 - F::new(4.0) * t1336 * t16132 * t5287 - t1336 * t19657 * t3856 + F::new(2.0) * t1336 * t40486 * t6388 - t12267 * t6456 + F::new(2.0) * t1332 * t20010 - F::new(2.0) * t16127 * t5234 - F::new(2.0) * t16423 * t5234 - F::new(2.0) * t19658 * t3777 - F::new(2.0) * t19813 * t3777 - t19815 * t3905 - t19815 * t3907 + t3773 * t6458;
    t57692
}
