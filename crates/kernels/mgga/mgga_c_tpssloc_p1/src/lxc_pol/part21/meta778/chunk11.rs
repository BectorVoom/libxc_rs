//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2701/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2701<F: Float>(t19815: F, t3802: F, t20000: F, t54566: F, t16398: F, t19873: F, t16397: F, t5234: F, t5252: F, t12429: F, t16244: F, t16265: F, t16383: F, t16394: F, t16401: F, t19871: F, t19966: F, t19986: F, t19991: F, t20004: F, t3803: F, t3805: F, t3809: F, t39993: F, t5246: F, t53958: F, t54125: F, t54131: F, t54133: F, t54135: F, t54138: F, t6394: F) -> F {
    let t56878 = t19815 * t3802;
    let t56883 = t54566 * t20000;
    let t56885 = t16398 * t19873;
    let t56888 = t5234 * t16397 * t5252;
    let t56904 = F::new(7.0) / F::new(1152.0) * t54125 + F::new(595.0) / F::new(864.0) * t54131 - F::new(35.0) / F::new(288.0) * t54133 - F::new(35.0) / F::new(288.0) * t54135 - F::new(35.0) / F::new(576.0) * t54138 + t12429 * t19986 / F::new(384.0) + t3803 * t3805 * t53958 * t6394 / F::new(384.0) + t56878 * t3809 / F::new(384.0) + t16394 * t16244 / F::new(192.0) + F::new(7.0) / F::new(384.0) * t56883 - F::new(7.0) / F::new(384.0) * t56885 - F::new(7.0) / F::new(576.0) * t56888 + t12429 * t19991 / F::new(192.0) - t16401 * t20004 / F::new(192.0) - t5246 * t3805 * t19871 * t39993 / F::new(384.0) + t16401 * t19966 / F::new(768.0) + t16394 * t16383 / F::new(384.0) - t16394 * t16265 / F::new(1536.0);
    t56904
}
