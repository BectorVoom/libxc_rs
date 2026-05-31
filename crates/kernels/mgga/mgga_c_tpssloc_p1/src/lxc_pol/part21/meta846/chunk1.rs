//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3062/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3062<F: Float>(t11424: F, t18255: F, t1117: F, t18835: F, t3264: F, t3307: F, t6021: F, t11190: F, t18258: F, t3265: F, t11185: F, t18259: F) -> (F, F, F, F, F) {
    let t63576 = F::cast_from(4.0_f64) * t11424 * t18255;
    let t63579 = F::cast_from(4.0_f64) * t3264 * t18835 * t1117;
    let t63582 = F::cast_from(2.0_f64) * t3264 * t6021 * t3307;
    let t63585 = F::cast_from(0.96491876992155210402e2_f64) * t11190 * t18258 * t3265;
    let t63587 = F::cast_from(0.32163958997385070134e2_f64) * t11185 * t18259;
    (t63576, t63579, t63582, t63585, t63587)
}
