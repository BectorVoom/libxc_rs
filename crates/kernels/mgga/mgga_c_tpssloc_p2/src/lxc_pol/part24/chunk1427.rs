//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1427/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1427<F: Float>(t22574: F, t39367: F, t8643: F, t22607: F, t6997: F, t12156: F, t1390: F, t1983: F, t2018: F, t22597: F, t6876: F, t22585: F) -> (F, F, F, F, F) {
    let t83869 = F::new(9.0) * t22574 * t8643 * t39367;
    let t83876 = F::new(3.0) * t22607 * t6997;
    let t83880 = F::new(6.0) * t1983 * t12156 * t2018 * t1390;
    let t83882 = F::new(18.0) * t6876 * t22597;
    let t83884 = F::new(9.0) * t6876 * t22585;
    (t83869, t83876, t83880, t83882, t83884)
}
