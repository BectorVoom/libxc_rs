//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1166/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1166<F: Float>(t118006: F, t24744: F, t24721: F, t7330: F, t7337: F, t24711: F, t8875: F, t32514: F, t7294: F, t2144: F, t7319: F, t1170: F, t2121: F, t32503: F) -> (F, F, F, F, F, F) {
    let t118007 = t24744 * t118006;
    let t118017 = t24721 * t7337 * t7330;
    let t118019 = t24711 * t8875;
    let t118034 = t7294 * t32514;
    let t118038 = t7319 * t2144;
    let t118050 = t2121 * t1170 * t32503;
    (t118007, t118017, t118019, t118034, t118038, t118050)
}
