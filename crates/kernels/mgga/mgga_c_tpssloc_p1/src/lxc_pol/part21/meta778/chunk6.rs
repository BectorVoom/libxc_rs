//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2696/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2696<F: Float>(t3791: F, t40046: F, t16398: F, t20004: F, t19945: F, t120: F, t1352: F, t16018: F, t16048: F, t16233: F, t16242: F, t19631: F, t19871: F, t19989: F, t3803: F, t3805: F, t5248: F, t5249: F, t53881: F, t53883: F, t53893: F, t53895: F, t53897: F, t53901: F, t53903: F, t53907: F, t53917: F, t53919: F, t54744: F, t550: F) -> (F, F) {
    let t56666 = t40046 * t3791;
    let t56685 = t16398 * t20004;
    let t56687 = t16398 * t19945;
    let t56689 = t3803 * t3805 * t16242 * t19989 / F::new(192.0) + t3803 * t3805 * t5249 * t550 * t16018 / F::new(384.0) + t3803 * t3805 * t120 * t19631 * t1352 / F::new(384.0) + t54744 * t5248 * t19871 * t56666 / F::new(128.0) - F::new(3.0) / F::new(256.0) * t16233 * t5248 * t19871 * t16048 - F::new(119.0) / F::new(864.0) * t53881 + F::new(7.0) / F::new(576.0) * t53883 + F::new(7.0) / F::new(576.0) * t53893 + F::new(7.0) / F::new(576.0) * t53895 + F::new(7.0) / F::new(288.0) * t53897 + F::new(595.0) / F::new(1296.0) * t53901 - F::new(35.0) / F::new(576.0) * t53903 + F::new(7.0) / F::new(288.0) * t53907 - F::new(119.0) / F::new(864.0) * t53917 - F::new(119.0) / F::new(864.0) * t53919 + F::new(7.0) / F::new(288.0) * t56685 - F::new(7.0) / F::new(576.0) * t56687;
    (t56666, t56689)
}
