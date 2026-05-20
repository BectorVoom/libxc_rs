//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2263/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2263<F: Float>(t46759: F, t46784: F, t46821: F, t46858: F, t225: F, t13242: F, t13244: F, t13254: F, t13265: F, t13316: F, t16836: F, t237: F, t249: F, t2633: F, t2643: F, t2679: F, t2684: F, t41066: F, t4178: F, t4180: F, t4181: F, t46717: F, t46733: F, t46737: F, t46742: F, t46748: F, t9629: F, t9642: F, t9958: F) -> (F, F, F) {
    let t46860 = t46759 + t46784 + t46821 + t46858;
    let t46861 = t46860 * t225;
    let t46868 = F::new(7.0) / F::new(768.0) * t46717 - t2643 * t4180 * t13242 * t2684 / F::new(1024.0) - t2643 * t4180 * t13242 * t2679 / F::new(1024.0) - t9642 * t13316 / F::new(1024.0) - t2643 * t4180 * t4181 * t9958 / F::new(3072.0) + F::new(7.0) / F::new(768.0) * t46733 - t16836 * t9629 / F::new(128.0) - F::new(3.0) / F::new(512.0) * t46737 * t13265 + F::new(7.0) / F::new(256.0) * t46742 + F::new(3.0) / F::new(512.0) * t4178 * t4180 * t13242 * t2633 - F::new(7.0) / F::new(256.0) * t46748 + t46861 * t237 * t249 / F::new(3072.0) + t13254 * t13244 / F::new(256.0) + F::new(35.0) / F::new(384.0) * t41066;
    (t46860, t46861, t46868)
}
