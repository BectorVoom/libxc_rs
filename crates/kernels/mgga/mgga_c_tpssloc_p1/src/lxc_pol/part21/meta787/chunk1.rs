//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2739/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2739<F: Float>(t20085: F, t3914: F, t39844: F, t5160: F, t57215: F, t57216: F, t57218: F, t57219: F, t57220: F, t57221: F, t57222: F, t57223: F, t57224: F, t57225: F) -> F {
    let t57815 = F::new(2.0) * t20085 * t3914 * t5160 + t39844 + t57215 - t57216 + t57218 - t57219 - t57220 + t57221 - t57222 - t57223 - t57224 - t57225;
    t57815
}
