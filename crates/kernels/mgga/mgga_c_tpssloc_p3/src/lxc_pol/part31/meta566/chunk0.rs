//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1797/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1797<F: Float>(t23133: F, t4257: F, t1496: F, t81942: F, t7497: F, t81933: F, t25098: F, t81835: F, t81877: F, t81883: F, t13176: F, t6620: F) -> (F, F, F, F, F, F, F) {
    let t87300 = t23133 * t4257;
    let t87304 = t81942 * t1496;
    let t87306 = t81933 * t7497;
    let t87308 = t81835 * t25098;
    let t87319 = F::cast_from(0.33643963411783659044e-4_f64) * t81877;
    let t87320 = F::cast_from(0.10541775202358879834e-2_f64) * t81883;
    let t87321 = t13176 * t6620;
    (t87300, t87304, t87306, t87308, t87319, t87320, t87321)
}
