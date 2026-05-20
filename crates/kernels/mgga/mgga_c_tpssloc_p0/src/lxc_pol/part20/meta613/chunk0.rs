//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2201/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2201<F: Float>(t35761: F, t35577: F, t112: F, t12512: F, t111: F, t3931: F, t16546: F, t576: F, t16506: F, t580: F, t2319: F, t4025: F) -> (F, F, F, F, F, F, F) {
    let t45460 = F::new(1.0) / t35761;
    let t45496 = F::new(1.0) / t35577;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45584 = t576 * t16546;
    let t45588 = t16506 * t580;
    let t45590 = t4025 * t2319;
    (t45460, t45496, t45557, t45560, t45584, t45588, t45590)
}
