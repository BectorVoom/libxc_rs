//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1229/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229<F: Float>(t172: F, t763: F, t9915: F, t184: F, t4194: F, t607: F, t9258: F, t12939: F, t2244: F, t9681: F, t2371: F, t9716: F) -> (F, F, F, F) {
    let t41265 = t9915 * t172 * t763;
    let t41266 = F::cast_from(0.23392894490538584828e1_f64) * t41265;
    let t41270 = F::new(48.0) * t4194 * t184 * t9258 * t607;
    let t41273 = F::new(144.0) * t12939 * t9681 * t2244;
    let t41274 = t9716 * t2371;
    (t41266, t41270, t41273, t41274)
}
