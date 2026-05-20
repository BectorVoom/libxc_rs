//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1428/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1428<F: Float>(t22573: F, t6875: F, t22575: F, t2319: F, t6514: F, t11968: F, t1869: F, t1976: F, t2312: F, t2320: F, t23829: F, t3929: F, t510: F, t650: F, t6862: F, t6872: F, t83692: F, t83694: F, t83698: F, t83853: F, t83862: F, t83866: F, t83869: F, t83876: F, t83880: F, t83882: F, t83884: F, t9347: F, t9351: F) -> (F, F) {
    let t83886 = t6875 * t22573;
    let t83888 = F::new(18.0) * t83886 * t22575;
    let t83889 = t6514 * t2319;
    let t83894 = -t11968 * t1869 - t1976 * t9347 - F::new(6.0) * t1976 * t9351 - F::new(3.0) * t2312 * t6862 - F::new(6.0) * t2320 * t6862 - F::new(3.0) * t23829 * t650 + F::new(3.0) * t3929 * t6872 - t510 * t83853 - F::new(6.0) * t510 * t83889 - t83692 - t83694 - t83698 + t83862 + t83866 - t83869 + t83876 + t83880 + t83882 + t83884 - t83888;
    (t83889, t83894)
}
