//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 649/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk649<F: Float>(t1877: F, t1915: F, t25: F, t2522: F, t606: F, t6542: F, t6666: F, t6670: F, t6671: F, t221: F, t60: F, t3: F, t607: F, t3034: F, t334: F, t371: F) -> (F, F, F, F, F) {
    let t6678 = 3.0 / 2.0 * t2522 * t1915 * t6542 + t1877 * t6666 * t25 / 2.0 - t1877 * t6670 * t6671 / 2.0 + t1877 * t1915 * t606 / 2.0;
    let t6686 = t221 * t60;
    let t6729 = t3 * t607;
    let t6739 = 1.0 / t3034 / t334;
    let t6793 = t371 * t334;
    (t6678, t6686, t6729, t6739, t6793)
}
