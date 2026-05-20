//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2774/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2774<F: Float>(t16673: F, t2642: F, t41424: F, t5587: F, t13278: F, t4236: F, t13186: F, t13196: F, t13222: F, t13251: F, t13254: F, t13306: F, t13316: F, t13350: F, t1510: F, t16891: F, t16893: F, t16896: F, t16924: F, t2633: F, t2643: F, t2649: F, t4172: F, t4178: F, t4180: F, t4182: F, t46698: F, t46717: F, t46733: F, t46742: F, t46748: F, t58495: F, t58552: F, t9632: F, t9642: F, t9646: F) -> F {
    let t58642 = t16673 * t2642;
    let t58668 = t41424 * t5587;
    let t58670 = t13278 * t4236;
    let t58672 = t9642 * t16924 / F::new(192.0) + t13254 * t16893 / F::new(768.0) + t4178 * t4180 * t58495 * t4182 / F::new(768.0) + t4178 * t4180 * t16891 * t9632 / F::new(1536.0) + t58642 * t2649 / F::new(384.0) - t13251 * t13316 / F::new(1536.0) + t13251 * t13306 / F::new(384.0) + F::new(5.0) / F::new(384.0) * t4178 * t9646 * t16896 * t2633 + t2643 * t13222 * t1510 * t58552 / F::new(192.0) - F::new(7.0) / F::new(288.0) * t46698 - F::new(5.0) / F::new(384.0) * t2643 * t13350 * t1510 * t13196 + F::new(7.0) / F::new(1152.0) * t46717 + F::new(7.0) / F::new(1152.0) * t46733 + F::new(7.0) / F::new(384.0) * t46742 - F::new(7.0) / F::new(384.0) * t46748 - F::new(5.0) / F::new(64.0) * t4172 * t13186 - F::new(7.0) / F::new(1152.0) * t58668 + F::new(7.0) / F::new(1152.0) * t58670;
    t58672
}
