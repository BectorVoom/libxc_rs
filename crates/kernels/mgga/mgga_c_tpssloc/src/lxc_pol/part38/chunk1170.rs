//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1170/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1170<F: Float>(t2: F, t8138: F, t29894: F, t29896: F, t29898: F, t29901: F, t29903: F, t30147: F, t30149: F, t30152: F, t30156: F, t30159: F, t30162: F, t30165: F, t30168: F, t30172: F, t30175: F, t8128: F, t8137: F) -> (F, F) {
    let t30176 = t8138 * t2;
    let t30179 = -t29894 - 2.0 / 3.0 * t29896 - 5.0 / 9.0 * t29898 + 5.0 / 9.0 * t29901 - 2.0 / 3.0 * t30147 - 3.0 / 4.0 * t29903 * t30149 - 5.0 / 12.0 * t8128 * t30152 + 5.0 / 12.0 * t8128 * t30156 + t8128 * t30159 / 4.0 + 5.0 / 9.0 * t30162 + 5.0 / 12.0 * t8128 * t30165 + 25.0 / 72.0 * t8137 * t30168 - 5.0 / 36.0 * t8137 * t30172 - 5.0 / 24.0 * t30175 * t30176;
    (t30176, t30179)
}
