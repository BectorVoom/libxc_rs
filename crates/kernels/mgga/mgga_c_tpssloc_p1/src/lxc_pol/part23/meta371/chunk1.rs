//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1172/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172<F: Float>(t11789: F, t820: F, t204: F, t486: F, t11716: F, t44833: F, t44834: F, t3503: F, t3584: F, t676: F, t221: F, t44483: F, t456: F) -> (F, F, F, F, F, F) {
    let t44951 = t820 * t11789;
    let t45017 = t204 * t486;
    let t45030 = t44833 * t11716 * t44834;
    let t45037 = t44833 * t3503 * t44834;
    let t45046 = t676 * t3584;
    let t45112 = F::new(5.0) / F::new(486.0) * t456 * t221 * t44483;
    (t44951, t45017, t45030, t45037, t45046, t45112)
}
