//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2726/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726<F: Float>(t16398: F, t19890: F, t12283: F, t19972: F, t225: F, t56570: F, t16150: F, t16155: F, t16233: F, t16308: F, t16387: F, t16394: F, t19855: F, t19871: F, t19876: F, t19956: F, t3803: F, t3851: F, t3858: F, t40335: F, t40443: F, t40449: F, t5240: F, t5248: F, t54764: F, t54785: F, t54787: F, t54793: F, t54801: F, t54811: F, t554: F, t559: F) -> (F, F) {
    let t57450 = t16398 * t19890;
    let t57457 = t12283 * t19972;
    let t57465 = t56570 * t225;
    let t57481 = t16394 * t16308 / F::new(192.0) + F::new(7.0) / F::new(144.0) * t57450 - F::new(7.0) / F::new(288.0) * t54764 - t16233 * t5248 * t19871 * t40335 / F::new(512.0) + F::new(7.0) / F::new(1152.0) * t57457 - t19855 * t3858 / F::new(3072.0) + F::new(5.0) / F::new(192.0) * t5240 * t16150 + F::new(5.0) / F::new(384.0) * t5240 * t16155 + t57465 * t554 * t559 / F::new(3072.0) + F::new(119.0) / F::new(3456.0) * t54785 + F::new(7.0) / F::new(72.0) * t54787 + t19876 * t16387 / F::new(256.0) - F::new(595.0) / F::new(5184.0) * t54793 + F::new(119.0) / F::new(13824.0) * t40443 + t40449 - F::new(7.0) / F::new(576.0) * t54801 - t3803 * t5248 * t19956 * t3851 / F::new(3072.0) + F::new(119.0) / F::new(1728.0) * t54811;
    (t57465, t57481)
}
