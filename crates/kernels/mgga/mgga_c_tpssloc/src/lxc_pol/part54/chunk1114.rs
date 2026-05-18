//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1114/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1114<F: Float>(t4724: F, t4899: F, t1210: F, t8039: F, t24721: F, t1714: F, t2133: F, t2132: F, t6739: F, t8026: F, t7325: F, t25588: F) -> (F, F, F, F, F) {
    let t27697 = t4899 * t4724;
    let t27700 = t1210 * t8039;
    let t27701 = t24721 * t27700;
    let t27703 = t2133 * t1714;
    let t27704 = t2132 * t27703;
    let t27710 = t8026 * t6739;
    let t27711 = t27710 * t7325;
    let t27714 = t2132 * t25588;
    (t27697, t27701, t27704, t27711, t27714)
}
