//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2191/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2191<F: Float>(t19731: F, t562: F, t16576: F, t751: F, t2517: F, t5520: F, t17109: F, t870: F, t16689: F, t2430: F, t12945: F, t4205: F) -> (F, F, F, F, F, F) {
    let t57704 = t562 * t19731;
    let t57887 = t16576 * t751;
    let t57897 = t5520 * t2517;
    let t57932 = t17109 * t870;
    let t57947 = t16689 * t2430;
    let t57960 = t4205 * t12945;
    (t57704, t57887, t57897, t57932, t57947, t57960)
}
