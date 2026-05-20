//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2354/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2354<F: Float>(t25971: F, t83886: F, t23831: F, t4028: F, t26504: F, t6876: F, t1983: F, t7687: F, t83929: F, t1874: F, t90370: F, t26114: F, t6525: F) -> (F, F, F, F, F, F) {
    let t91578 = F::new(6.0) * t83886 * t25971;
    let t91580 = F::new(2.0) * t4028 * t23831;
    let t91582 = F::new(2.0) * t6876 * t26504;
    let t91585 = F::new(3.0) * t1983 * t83929 * t7687;
    let t91587 = F::new(4.0) * t90370 * t1874;
    let t91589 = F::new(4.0) * t26114 * t6525;
    (t91578, t91580, t91582, t91585, t91587, t91589)
}
