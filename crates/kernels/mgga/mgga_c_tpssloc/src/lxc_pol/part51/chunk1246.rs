//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1246/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1246<F: Float>(t112936: F, t112942: F, t114916: F, t114933: F, t114939: F, t13042: F, t1492: F, t24305: F, t25348: F, t259: F, t26700: F, t31343: F, t31361: F, t31423: F, t4142: F, t4147: F, t4273: F, t6632: F, t7092: F, t7538: F, t8543: F, t8553: F) -> (F,) {
    let t121711 = 0.82246703342411321824e-2 * t114916 + t4142 * t8543 * t259 + t1492 * t31361 * t259 + 2.0 * t13042 * t8553 + 2.0 * t25348 * t7092 + 2.0 * t4147 * t31343 + t112936 + 2.0 * t26700 * t6632 - t24305 * t7538 + 2.0 * t31423 * t4273 - t114933 - t112942 + 0.19190897446562641759e-1 * t114939;
    (t121711,)
}
