//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2260/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2260<F: Float>(t22574: F, t56120: F, t8643: F, t1845: F, t3719: F, t1874: F, t55962: F, t19456: F, t6525: F, t22480: F, t4028: F, t26502: F, t532: F) -> (F, F, F, F, F, F) {
    let t91602 = F::new(3.0) * t22574 * t8643 * t56120;
    let t91603 = t1845 * t3719;
    let t91606 = F::new(3.0) * t22574 * t8643 * t91603;
    let t91608 = F::new(2.0) * t55962 * t1874;
    let t91610 = F::new(4.0) * t19456 * t6525;
    let t91612 = F::new(2.0) * t4028 * t22480;
    let t91620 = t532 * t26502;
    (t91602, t91606, t91608, t91610, t91612, t91620)
}
