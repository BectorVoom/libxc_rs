//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 693/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk693<F: Float>(t1193: F, t8020: F, t52: F, t8027: F, t2136: F, t461: F, t7573: F, t7324: F, t1210: F, t8039: F, t24721: F, t6739: F, t8026: F, t7325: F, t24574: F, t8070: F) -> (F, F, F, F, F, F) {
    let t27677 = t8020 * t1193;
    let t27680 = t8027 * t52;
    let t27681 = t27680 * t2136;
    let t27683 = t7573 * t461;
    let t27684 = t7324 * t27683;
    let t27700 = t1210 * t8039;
    let t27701 = t24721 * t27700;
    let t27710 = t8026 * t6739;
    let t27711 = t27710 * t7325;
    let t27728 = t24574 * t8070;
    (t27677, t27681, t27684, t27701, t27711, t27728)
}
