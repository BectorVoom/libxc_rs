//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2094/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2094<F: Float>(t11913: F, t44690: F, t11604: F, t496: F, t68: F, t107: F, t9576: F, t2585: F, t667: F, t106: F, t9364: F, t35761: F) -> (F, F, F, F, F, F) {
    let t45329 = t44690 * t11913;
    let t45349 = F::new(1.0) / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45421 = F::new(2618.0) / F::new(81.0) * t9576 * t107;
    let t45422 = t2585 * t667;
    let t45435 = F::new(1.0) / t9364 / t106;
    let t45460 = F::new(1.0) / t35761;
    (t45329, t45350, t45421, t45422, t45435, t45460)
}
