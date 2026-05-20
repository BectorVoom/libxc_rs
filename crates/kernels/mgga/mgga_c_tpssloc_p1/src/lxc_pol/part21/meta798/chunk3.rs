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
    let t58725 = t2630 * t819 * t820 * t58340 / F::new(768.0) - F::new(5.0) / F::new(128.0) * t843 * t9607 * t820 * t5544 * t2379 - F::new(7.0) / F::new(576.0) * t46870 + F::new(119.0) / F::new(1728.0) * t46874 - t2643 * t46692 * t13223 * t4234 / F::new(768.0) + t2643 * t13222 * t58688 * t2647 / F::new(384.0) - F::new(5.0) / F::new(192.0) * t13251 * t13353 - t9613 * t5614 / F::new(3072.0) - t2618 * t16859 / F::new(1536.0) - t817 * t819 * t820 * t58281 / F::new(1536.0) - F::new(5.0) / F::new(64.0) * t2623 * t16853 - F::new(35.0) / F::new(288.0) * t58705 - t47220 * t1512 / F::new(1536.0) - F::new(35.0) / F::new(576.0) * t58709 + F::new(5.0) / F::new(384.0) * t2623 * t16951 + F::new(5.0) / F::new(384.0) * t843 * t2701 * t820 * t16662 * t776 + F::new(5.0) / F::new(768.0) * t843 * t2701 * t820 * t5544 * t2553 - F::new(119.0) / F::new(13824.0) * t58723;
    t58725
}
