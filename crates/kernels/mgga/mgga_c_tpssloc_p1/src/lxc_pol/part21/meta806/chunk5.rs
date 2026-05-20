//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2803/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2803<F: Float>(t59134: F, t59178: F, t59197: F, t59227: F, t225: F, t13222: F, t13223: F, t13228: F, t16912: F, t16969: F, t210: F, t237: F, t2379: F, t249: F, t2643: F, t41130: F, t41134: F, t41139: F, t41161: F, t41341: F, t41363: F, t41365: F, t41373: F, t41386: F, t4178: F, t46692: F, t47017: F, t47093: F, t47230: F, t47267: F, t5567: F, t5571: F, t59100: F, t9559: F, t9642: F) -> (F, F, F) {
    let t59229 = t59134 + t59178 + t59197 + t59227;
    let t59230 = t59229 * t225;
    let t59235 = -F::new(595.0) / F::new(5184.0) * t41130 + F::new(119.0) / F::new(13824.0) * t41134 + t41139 - F::new(119.0) / F::new(6912.0) * t41341 - F::new(119.0) / F::new(3456.0) * t47093 + t9642 * t16969 / F::new(192.0) + F::new(595.0) / F::new(5184.0) * t41363 - F::new(119.0) / F::new(13824.0) * t41365 - F::new(119.0) / F::new(13824.0) * t41373 + F::new(119.0) / F::new(6912.0) * t41386 - F::new(35.0) / F::new(54.0) * t47230 + F::new(7.0) / F::new(6.0) * t59100 + F::new(5.0) / F::new(4.0) * t41161 * t210 * t5567 * t2379 - t9559 * t210 * t5571 * t2379 / F::new(4.0) + t2643 * t13222 * t13223 * t16912 / F::new(192.0) + t4178 * t46692 * t13228 * t47017 / F::new(128.0) + t59230 * t237 * t249 / F::new(3072.0) - F::new(35.0) / F::new(576.0) * t47267;
    (t59229, t59230, t59235)
}
