//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2437/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2437<F: Float>(t13520: F, t17507: F, t13727: F, t17510: F, t10661: F, t4395: F, t5730: F, t21303: F, t42028: F, t912: F, t21300: F, t2792: F) -> (F, F, F, F, F) {
    let t69335 = F::new(18.0) * t13520 * t17507;
    let t69337 = F::new(12.0) * t13727 * t17510;
    let t69340 = F::cast_from(0.28947563097646563121e3_f64) * t10661 * t5730 * t4395;
    let t69343 = F::cast_from(0.62071215503128080361e4_f64) * t42028 * t21303 * t912;
    let t69346 = F::new(2.0) * t2792 * t21300 * t912;
    (t69335, t69337, t69340, t69343, t69346)
}
