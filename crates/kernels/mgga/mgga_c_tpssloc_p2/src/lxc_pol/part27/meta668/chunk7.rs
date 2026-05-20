//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2362/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2362<F: Float>(t1774: F, t22479: F, t652: F, t7468: F, t9348: F, t15904: F, t22574: F, t33136: F, t12734: F, t2314: F, t26003: F, t1874: F, t90381: F) -> (F, F, F, F, F, F) {
    let t91713 = F::new(2.0) * t652 * t1774 * t22479;
    let t91715 = F::new(2.0) * t9348 * t7468;
    let t91718 = F::new(6.0) * t22574 * t33136 * t15904;
    let t91722 = F::new(4.0) * t12734 * t7468;
    let t91724 = F::new(4.0) * t2314 * t26003;
    let t91726 = F::new(2.0) * t90381 * t1874;
    (t91713, t91715, t91718, t91722, t91724, t91726)
}
