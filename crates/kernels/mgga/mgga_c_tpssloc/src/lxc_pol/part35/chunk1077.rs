//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1077/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1077<F: Float>(t2132: F, t27703: F, t6739: F, t8026: F, t7325: F, t24574: F, t8070: F, t1170: F, t8077: F, t2121: F, t1751: F, t7299: F, t8015: F, t8006: F, t3242: F, t497: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27704 = t2132 * t27703;
    let t27710 = t8026 * t6739;
    let t27711 = t27710 * t7325;
    let t27728 = t24574 * t8070;
    let t27736 = t1170 * t8077;
    let t27737 = t2121 * t27736;
    let t27751 = t7299 * t1751;
    let t27755 = t24574 * t8015;
    let t27770 = t24574 * t8006;
    let t27774 = t497 * t3242;
    (t27704, t27710, t27711, t27728, t27736, t27737, t27751, t27755, t27770, t27774)
}
