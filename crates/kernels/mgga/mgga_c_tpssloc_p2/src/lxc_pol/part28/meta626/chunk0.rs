//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1952/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1952<F: Float>(t6875: F, t8944: F, t1845: F, t3698: F, t3734: F, t12813: F, t89: F, t27240: F, t580: F, t1395: F, t7961: F, t1851: F, t7240: F) -> (F, F, F, F, F, F, F) {
    let t91669 = t6875 * t8944;
    let t91687 = t1845 * t3698;
    let t91695 = t1845 * t3734;
    let t91753 = t89 * t12813;
    let t91830 = F::new(2.0) * t27240 * t580;
    let t91832 = F::new(2.0) * t1395 * t7961;
    let t91834 = F::new(2.0) * t1851 * t7240;
    (t91669, t91687, t91695, t91753, t91830, t91832, t91834)
}
