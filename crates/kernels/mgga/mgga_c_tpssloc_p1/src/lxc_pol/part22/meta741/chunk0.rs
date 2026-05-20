//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2443/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2443<F: Float>(t13662: F, t5791: F, t959: F, t21095: F, t2940: F, t17202: F, t4696: F, t4700: F, t69036: F, t69253: F, t69255: F, t69257: F, t69259: F, t69261: F, t69453: F, t69456: F) -> (F, F, F) {
    let t69459 = F::cast_from(0.35089341735807877242e1_f64) * t959 * t13662 * t5791;
    let t69461 = F::cast_from(0.10389515463408878255e3_f64) * t2940 * t21095;
    let t69462 = -F::new(3.0) * t17202 * t4696 * t4700 + t69036 - t69253 + t69255 - t69257 + t69259 + t69261 + t69453 + t69456 + t69459 + t69461;
    (t69459, t69461, t69462)
}
