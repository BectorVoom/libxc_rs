//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 841/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk841<F: Float>(t6883: F, t8480: F, t2006: F, t552: F, t794: F, t8479: F, t6897: F, t8537: F, t6562: F, t2053: F, t2717: F, t857: F) -> (F, F, F, F, F, F, F, F) {
    let t31192 = F::new(0.38381794893125283518e-1) * t6883 * t8480;
    let t31193 = t552 * t2006;
    let t31198 = t794 * t8479;
    let t31200 = F::new(0.82246703342411321825e-2) * t6897 * t31198;
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31321 = F::new(0.41123351671205660912e-2) * t31320;
    let t31332 = t2717 * t2053;
    let t31337 = t857 * t2053;
    (t31192, t31193, t31198, t31200, t31319, t31321, t31332, t31337)
}
