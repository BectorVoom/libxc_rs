//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1021/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1021<F: Float>(t114916: F, t114939: F, t116645: F, t116648: F, t121689: F, t121713: F, t121716: F, t123641: F, t123663: F, t13053: F, t13065: F, t1492: F, t1528: F, t2054: F, t259: F, t26582: F, t26680: F, t2718: F, t31984: F, t33973: F, t4142: F, t7087: F, t855: F, t858: F, t865: F, t8728: F, t8734: F, t92386: F) -> F {
    let t123687 = F::new(2.0) * t855 * t2718 * t33973 * t865 - t855 * t858 * (t123641 + t123663) + t4142 * t8728 * t259 + t1492 * t31984 * t259 + F::new(0.6579736267392905746e-1) * t121689 + F::new(0.3289868133696452873e-1) * t114916 - t116645 * t1528 + F::new(2.0) * t13065 * t8734 - F::new(2.0) * t92386 * t2054 - t116648 + F::new(0.76763589786250567037e-1) * t114939 - F::new(2.0) * t7087 * t26680 - F::new(0.3289868133696452873e-1) * t121713 - F::new(0.3289868133696452873e-1) * t121716 + F::new(4.0) * t7087 * t26582 + F::new(2.0) * t13053 * t8734;
    t123687
}
