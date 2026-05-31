//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1111/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1111<F: Float>(t1512: F, t2639: F, t249: F, t2571: F, t2602: F, t2603: F, t2618: F, t4152: F, t4155: F, t4159: F, t4163: F, t4167: F, t4170: F, t4172: F, t4178: F, t4184: F, t787: F, t831: F, t849: F) -> (F, F) {
    let t4187 = t2639 * t1512;
    let t4189 = t2602 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2603 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t4152 + t2571 * t4155 / F::cast_from(16.0_f64) - t787 * t4159 / F::cast_from(48.0_f64) + t4163 * t249 / F::cast_from(3072.0_f64) - t4167 * t831 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t4170 - t4172 * t849 / F::cast_from(768.0_f64) - t2618 * t1512 / F::cast_from(3072.0_f64) + t4178 * t4184 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t4187;
    (t4187, t4189)
}
