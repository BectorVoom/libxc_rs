//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1275/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1275<F: Float>(t30164: F, t8138: F, t1444: F, t29922: F, t659: F, t29926: F, t2585: F, t656: F, t2: F, t29894: F, t29896: F, t29898: F, t29901: F, t29903: F, t30147: F, t30149: F, t30152: F, t30156: F, t30159: F, t30162: F, t8128: F, t8137: F) -> (F, F, F, F, F, F, F) {
    let t30165 = t8138 * t30164;
    let t30168 = t29922 * t1444;
    let t30171 = t1444 * t659;
    let t30172 = t29926 * t30171;
    let t30175 = t2585 * t656;
    let t30176 = t8138 * t2;
    let t30179 = -t29894 - F::new(2.0) / F::new(3.0) * t29896 - F::new(5.0) / F::new(9.0) * t29898 + F::new(5.0) / F::new(9.0) * t29901 - F::new(2.0) / F::new(3.0) * t30147 - F::new(3.0) / F::new(4.0) * t29903 * t30149 - F::new(5.0) / F::new(12.0) * t8128 * t30152 + F::new(5.0) / F::new(12.0) * t8128 * t30156 + t8128 * t30159 / F::new(4.0) + F::new(5.0) / F::new(9.0) * t30162 + F::new(5.0) / F::new(12.0) * t8128 * t30165 + F::new(25.0) / F::new(72.0) * t8137 * t30168 - F::new(5.0) / F::new(36.0) * t8137 * t30172 - F::new(5.0) / F::new(24.0) * t30175 * t30176;
    (t30165, t30168, t30171, t30172, t30175, t30176, t30179)
}
