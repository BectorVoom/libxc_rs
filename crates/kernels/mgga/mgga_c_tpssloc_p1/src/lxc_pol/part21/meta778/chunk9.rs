//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2699/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2699<F: Float>(t16336: F, t5314: F, t1831: F, t53880: F, t19930: F, t3866: F, t1351: F, t5187: F, t6414: F, t120: F, t19731: F, t12336: F, t12429: F, t1363: F, t1367: F, t16227: F, t16248: F, t16305: F, t16311: F, t16321: F, t16394: F, t19871: F, t19958: F, t3783: F, t3793: F, t3803: F, t3807: F, t5246: F, t5248: F, t5250: F, t53910: F, t54047: F, t54059: F, t56275: F, t6427: F, t6431: F, t820: F) -> (F, F) {
    let t56779 = t16336 * t5314;
    let t56795 = t53880 * t1831;
    let t56797 = t3866 * t19930;
    let t56805 = t5187 * t1351;
    let t56812 = t6414 * t1351;
    let t56817 = t120 * t19731;
    let t56826 = F::new(7.0) / F::new(288.0) * t56779 - t53910 * t1831 / F::new(384.0) - t16321 * t5314 / F::new(192.0) + F::new(5.0) / F::new(768.0) * t12336 * t6427 - t12336 * t6431 / F::new(768.0) - t3783 * t19930 / F::new(384.0) - t1363 * t1367 * t820 * t56275 / F::new(768.0) - F::new(119.0) / F::new(1728.0) * t56795 + F::new(7.0) / F::new(576.0) * t56797 - F::new(119.0) / F::new(3456.0) * t54047 + F::new(35.0) / F::new(576.0) * t54059 + F::new(7.0) / F::new(1536.0) * t5246 * t5248 * t19871 * t3793 - t5246 * t16305 * t16311 * t56805 / F::new(96.0) - F::new(5.0) / F::new(192.0) * t16394 * t16227 + t3803 * t16305 * t56812 * t3807 / F::new(384.0) + t5246 * t5248 * t56817 * t5250 / F::new(768.0) + t16394 * t16248 / F::new(384.0) + t12429 * t19958 / F::new(384.0);
    (t56817, t56826)
}
