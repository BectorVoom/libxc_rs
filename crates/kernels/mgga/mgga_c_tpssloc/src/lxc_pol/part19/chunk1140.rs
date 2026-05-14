//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1140/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1140<F: Float>(t41265: F, t184: F, t4194: F, t607: F, t9258: F, t12939: F, t2244: F, t9681: F, t2371: F, t9716: F, t185: F, t39110: F, t707: F, t2447: F, t32: F, t2659: F) -> (F, F, F, F, F, F) {
    let t41266 = 0.23392894490538584828e1 * t41265;
    let t41270 = 48.0 * t4194 * t184 * t9258 * t607;
    let t41273 = 144.0 * t12939 * t9681 * t2244;
    let t41274 = t9716 * t2371;
    let t41275 = 0.70178683471615754484e1 * t41274;
    let t41278 = 4.0 * t707 * t185 * t39110;
    let t41279 = t32 * t2447;
    let t41281 = 72.0 * t41279 * t2659;
    (t41266, t41270, t41273, t41275, t41278, t41281)
}
