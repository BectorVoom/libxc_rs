//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 581/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk581<F: Float>(t117: F, t1540: F, t321: F, t325: F, t446: F, t618: F, t622: F, t1343: F, t7321: F, t7334: F, t7552: F, t7203: F, t892: F, t899: F, t20: F, t4764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30221 = t1540 * t117;
    let t30526 = t321 * t325;
    let t31817 = t446 * t618;
    let t33235 = t622 * t321;
    let t34683 = t7321 * t1343;
    let t34709 = t7334 * t7552;
    let t34735 = t892 * t7203;
    let t34738 = t899 * t7203;
    let t34747 = t20 * t4764;
    (t30221, t30526, t31817, t33235, t34683, t34709, t34735, t34738, t34747)
}
