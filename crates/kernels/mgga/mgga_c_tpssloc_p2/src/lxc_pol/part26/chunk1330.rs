//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1330/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1330<F: Float>(t22607: F, t6997: F, t12156: F, t1390: F, t1983: F, t2018: F, t22597: F, t6876: F, t22585: F, t22573: F, t6875: F, t22575: F) -> (F, F, F, F, F) {
    let t83876 = F::cast_from(3.0_f64) * t22607 * t6997;
    let t83880 = F::cast_from(6.0_f64) * t1983 * t12156 * t2018 * t1390;
    let t83882 = F::cast_from(18.0_f64) * t6876 * t22597;
    let t83884 = F::cast_from(9.0_f64) * t6876 * t22585;
    let t83886 = t6875 * t22573;
    let t83888 = F::cast_from(18.0_f64) * t83886 * t22575;
    (t83876, t83880, t83882, t83884, t83888)
}
