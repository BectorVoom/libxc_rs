//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2231/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2231<F: Float>(t13822: F, t17777: F, t973: F, t2986: F, t4514: F, t48019: F, t48046: F, t10236: F, t17691: F, t13779: F, t17183: F, t16558: F, t2989: F) -> (F, F, F, F, F, F) {
    let t61472 = t973 * t13822 * t17777;
    let t61489 = t2986 * t48019 * t4514;
    let t61495 = t2986 * t48046 * t4514;
    let t61528 = t10236 * t17691;
    let t61557 = t2986 * t13779 * t17183;
    let t61589 = t2989 * t16558;
    (t61472, t61489, t61495, t61528, t61557, t61589)
}
