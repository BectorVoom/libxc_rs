//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 445/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk445<F: Float>(t120: F, t1509: F, t2632: F, t828: F, t4180: F, t1512: F, t2639: F, t249: F, t2571: F, t2602: F, t2603: F, t2618: F, t4152: F, t4155: F, t4159: F, t4163: F, t4167: F, t4170: F, t4172: F, t4178: F, t787: F, t831: F, t849: F) -> (F, F, F, F) {
    let t4181 = t120 * t1509;
    let t4182 = t2632 * t828;
    let t4184 = t4180 * t4181 * t4182;
    let t4187 = t2639 * t1512;
    let t4189 = t2602 + 7.0 / 144.0 * t2603 + 7.0 / 144.0 * t4152 + t2571 * t4155 / 16.0 - t787 * t4159 / 48.0 + t4163 * t249 / 3072.0 - t4167 * t831 / 3072.0 - 7.0 / 4608.0 * t4170 - t4172 * t849 / 768.0 - t2618 * t1512 / 3072.0 + t4178 * t4184 / 1536.0 + 7.0 / 4608.0 * t4187;
    (t4181, t4182, t4184, t4189)
}
