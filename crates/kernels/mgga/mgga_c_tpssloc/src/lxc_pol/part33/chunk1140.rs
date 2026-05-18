//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1140/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1140<F: Float>(t22759: F, t242: F, t1336: F, t1887: F, t22839: F, t1799: F, t567: F, t1377: F, t22674: F, t7700: F, t6897: F, t1842: F, t3886: F) -> (F, F, F, F, F, F, F) {
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    let t26331 = t22839 * t1887;
    let t26332 = t567 * t1799;
    let t26337 = t1377 * t1799;
    let t26344 = t22674 * t7700;
    let t26345 = t6897 * t26344;
    let t26354 = t3886 * t1842;
    (t26309, t26331, t26332, t26337, t26344, t26345, t26354)
}
