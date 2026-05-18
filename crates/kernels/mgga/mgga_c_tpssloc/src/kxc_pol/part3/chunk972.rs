//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 972/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk972<F: Float>(t3788: F, t835: F, t1336: F, t3795: F, t3799: F, t3853: F, t3858: F, t12267: F, t1340: F, t3719: F, t550: F, t1995: F, t67: F) -> (F, F, F, F, F, F) {
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12386 = t12385 * t3795;
    let t12388 = t3799 * t3853;
    let t12395 = t3799 * t3858;
    let t12397 = t12267 * t1340;
    let t12407 = t550 * t3719;
    let t12418 = t1995 * t67;
    (t12386, t12388, t12395, t12397, t12407, t12418)
}
