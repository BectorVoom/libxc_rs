//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2252/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2252<F: Float>(t13176: F, t2638: F, t831: F, t13251: F, t13350: F, t2643: F, t2645: F, t2647: F, t41048: F, t41050: F, t41053: F, t41055: F, t41063: F, t4191: F, t4248: F, t4257: F, t46644: F, t46650: F, t46658: F, t46661: F, t46663: F, t9623: F, t9661: F, t9990: F) -> F {
    let t46667 = t13176 * t2638;
    let t46668 = t46667 * t831;
    let t46670 = t2643 * t2645 * t4248 * t9661 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t41048 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t41050 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t13350 * t46644 * t2647 + t46650 + t41063 * t4191 / F::cast_from(256.0_f64) - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t41053 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t41055 - t13251 * t9623 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t46658 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t46661 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t46663 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t9990 * t4257 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t46668;
    t46670
}
