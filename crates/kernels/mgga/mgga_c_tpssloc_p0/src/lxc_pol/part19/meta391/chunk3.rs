//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1474/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1474<F: Float>(t221: F, t44483: F, t456: F, t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F, t11669: F, t3577: F, t11779: F) -> (F, F, F, F, F, F) {
    let t45112 = F::new(5.0) / F::new(486.0) * t456 * t221 * t44483;
    let t45113 = t3575 * t42386;
    let t45114 = t11888 * t45113;
    let t45119 = t11914 * t45113;
    let t45124 = t820 * t11784;
    let t45126 = t3577 * t45124 * t11669;
    let t45128 = t820 * t11779;
    (t45112, t45113, t45114, t45119, t45126, t45128)
}
