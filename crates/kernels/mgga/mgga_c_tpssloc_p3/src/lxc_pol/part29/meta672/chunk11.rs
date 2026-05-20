//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2259/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2259<F: Float>(t26504: F, t6876: F, t1983: F, t7687: F, t83929: F, t1874: F, t90370: F, t26114: F, t6525: F, t12734: F, t7461: F, t2314: F, t25980: F) -> (F, F, F, F, F, F) {
    let t91582 = F::new(2.0) * t6876 * t26504;
    let t91585 = F::new(3.0) * t1983 * t83929 * t7687;
    let t91587 = F::new(4.0) * t90370 * t1874;
    let t91589 = F::new(4.0) * t26114 * t6525;
    let t91591 = F::new(4.0) * t12734 * t7461;
    let t91593 = F::new(4.0) * t2314 * t25980;
    (t91582, t91585, t91587, t91589, t91591, t91593)
}
