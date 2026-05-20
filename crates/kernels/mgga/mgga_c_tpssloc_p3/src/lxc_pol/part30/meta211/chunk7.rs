//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 999/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk999<F: Float>(t1512: F, t1516: F, t249: F, t2571: F, t2602: F, t2630: F, t2643: F, t2695: F, t4152: F, t4167: F, t4170: F, t4172: F, t4187: F, t4253: F, t5568: F, t5572: F, t5576: F, t5587: F, t5593: F, t5614: F, t5619: F, t5624: F, t5628: F, t787: F, t817: F, t843: F) -> F {
    let t5631 = t2602 + F::new(7.0) / F::new(72.0) * t4152 + t2571 * t5568 / F::new(16.0) - t787 * t5572 / F::new(48.0) + t5576 * t249 / F::new(3072.0) - t4167 * t1512 / F::new(1536.0) - F::new(7.0) / F::new(2304.0) * t4170 - t4172 * t1516 / F::new(384.0) + t2630 * t5587 / F::new(1536.0) + F::new(7.0) / F::new(2304.0) * t4187 + t2643 * t5593 / F::new(384.0) - t817 * t5614 / F::new(3072.0) - t817 * t5619 / F::new(3072.0) + t2695 + F::new(7.0) / F::new(576.0) * t4253 + F::new(5.0) / F::new(768.0) * t843 * t5624 - t843 * t5628 / F::new(768.0);
    t5631
}
