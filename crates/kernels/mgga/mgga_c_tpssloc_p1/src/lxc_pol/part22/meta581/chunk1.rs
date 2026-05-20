//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2091/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2091<F: Float>(t3503: F, t44833: F, t44834: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F, t221: F, t44483: F, t456: F, t3575: F, t42386: F) -> (F, F, F, F, F) {
    let t45037 = t44833 * t3503 * t44834;
    let t45044 = t1174 * t2402 * t1197;
    let t45046 = t676 * t3584;
    let t45112 = F::new(5.0) / F::new(486.0) * t456 * t221 * t44483;
    let t45113 = t3575 * t42386;
    (t45037, t45044, t45046, t45112, t45113)
}
