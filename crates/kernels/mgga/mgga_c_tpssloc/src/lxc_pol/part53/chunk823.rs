//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 823/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk823<F: Float>(t1492: F, t8728: F, t31976: F, t31978: F, t31982: F, t32835: F, t32838: F, t32841: F, t32845: F, t32847: F, t218: F, t10110: F, t1527: F, t8733: F, t259: F, t31971: F, t32014: F, t33372: F, t33410: F, t33420: F, t33423: F, t33430: F, t33935: F, t7087: F, t7830: F, t7842: F, t855: F) -> (F, F, F, F, F) {
    let t33940 = t1492 * t8728;
    let t33947 = -t31976 - 0.19378922925187387609e-1 * t32835 - t31978 - 0.32298204875312312682e-2 * t32838 + t32841 / 384.0 - t32845 / 384.0 - t31982 - t32847 / 96.0;
    let t33948 = t218 * t33947;
    let t33951 = t10110 * t8733 * t1527;
    let t33960 = -0.3289868133696452873e-1 * t33372 - t31971 + 4.0 * t855 * t33935 + 4.0 * t7087 * t7830 + t33940 * t259 + t33948 * t259 - 6.0 * t855 * t33951 - 0.3289868133696452873e-1 * t33410 - t32014 - 0.6579736267392905746e-1 * t33420 - 0.3289868133696452873e-1 * t33423 + 0.3289868133696452873e-1 * t33430 - 2.0 * t7087 * t7842;
    (t33940, t33947, t33948, t33951, t33960)
}
