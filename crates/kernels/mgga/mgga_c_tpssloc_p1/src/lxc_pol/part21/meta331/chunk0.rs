//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1706/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1706<F: Float>(t12365: F, t1354: F, t1307: F, t3792: F, t3788: F, t835: F, t1336: F, t3795: F, t3799: F, t3853: F, t3858: F, t12267: F, t1340: F) -> (F, F, F, F, F, F, F, F) {
    let t12366 = t12365 * t1354;
    let t12369 = t3792 * t1307;
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12386 = t12385 * t3795;
    let t12388 = t3799 * t3853;
    let t12395 = t3799 * t3858;
    let t12397 = t12267 * t1340;
    (t12366, t12369, t12384, t12385, t12386, t12388, t12395, t12397)
}
