//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2775/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2775<F: Float>(t5584: F, t828: F, t16946: F, t2697: F, t16951: F, t5614: F, t9671: F, t13222: F, t13223: F, t13251: F, t13353: F, t1512: F, t16662: F, t16853: F, t16859: F, t2379: F, t2553: F, t2618: F, t2623: F, t2630: F, t2643: F, t2647: F, t2701: F, t4234: F, t46692: F, t46870: F, t46874: F, t47220: F, t5544: F, t58281: F, t58340: F, t776: F, t817: F, t819: F, t820: F, t843: F, t9607: F, t9613: F) -> F {
    let t58688 = t5584 * t828;
    let t58705 = t2697 * t16946;
    let t58709 = t2697 * t16951;
    let t58723 = t9671 * t5614;
    let t58725 = t2630 * t819 * t820 * t58340 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t843 * t9607 * t820 * t5544 * t2379 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t46870 + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t46874 - t2643 * t46692 * t13223 * t4234 / F::cast_from(768.0_f64) + t2643 * t13222 * t58688 * t2647 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t13251 * t13353 - t9613 * t5614 / F::cast_from(3072.0_f64) - t2618 * t16859 / F::cast_from(1536.0_f64) - t817 * t819 * t820 * t58281 / F::cast_from(1536.0_f64) - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2623 * t16853 - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t58705 - t47220 * t1512 / F::cast_from(1536.0_f64) - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t58709 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t2623 * t16951 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t843 * t2701 * t820 * t16662 * t776 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t843 * t2701 * t820 * t5544 * t2553 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t58723;
    t58725
}
