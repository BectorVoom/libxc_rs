//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 777/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk777<F: Float>(t4180: F, t4181: F, t4182: F, t1512: F, t2639: F, t249: F, t2571: F, t2602: F, t2603: F, t2618: F, t4152: F, t4155: F, t4159: F, t4163: F, t4167: F, t4170: F, t4172: F, t4178: F, t787: F, t831: F, t849: F) -> (F, F) {
    let t4184 = t4180 * t4181 * t4182;
    let t4187 = t2639 * t1512;
    let t4189 = t2602 + F::new(7.0) / F::new(144.0) * t2603 + F::new(7.0) / F::new(144.0) * t4152 + t2571 * t4155 / F::new(16.0) - t787 * t4159 / F::new(48.0) + t4163 * t249 / F::new(3072.0) - t4167 * t831 / F::new(3072.0) - F::new(7.0) / F::new(4608.0) * t4170 - t4172 * t849 / F::new(768.0) - t2618 * t1512 / F::new(3072.0) + t4178 * t4184 / F::new(1536.0) + F::new(7.0) / F::new(4608.0) * t4187;
    (t4184, t4189)
}
