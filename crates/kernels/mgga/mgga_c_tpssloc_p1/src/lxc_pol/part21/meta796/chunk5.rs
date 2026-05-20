//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2763/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2763<F: Float>(t1509: F, t4265: F, t13336: F, t13393: F, t13450: F, t13453: F, t1510: F, t1525: F, t16756: F, t16758: F, t16815: F, t16817: F, t16820: F, t16825: F, t16830: F, t17031: F, t17034: F, t2617: F, t2679: F, t2684: F, t4291: F, t47395: F, t47419: F, t5651: F, t812: F, t829: F, t9612: F) -> (F, F) {
    let t58204 = t4265 * t1509;
    let t58224 = -F::new(2.0) * t1510 * t47395 * t812 - F::new(2.0) * t16758 * t2684 * t4291 - t16815 * t2679 * t4291 - F::new(4.0) * t4291 * t58204 * t829 + F::new(2.0) * t13336 * t1525 + F::new(8.0) * t13393 * t17034 - F::new(2.0) * t13450 * t16830 + F::new(8.0) * t13453 * t16820 + F::new(12.0) * t13453 * t16825 + F::new(4.0) * t13453 * t17031 - F::new(2.0) * t16756 * t2617 - F::new(12.0) * t16817 * t47419 - t5651 * t9612;
    (t58204, t58224)
}
