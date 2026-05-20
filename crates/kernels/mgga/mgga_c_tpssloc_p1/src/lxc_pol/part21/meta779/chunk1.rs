//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2703/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2703<F: Float>(t1340: F, t56923: F, t12365: F, t6417: F, t12283: F, t19962: F, t19882: F, t19996: F, t3866: F, t40018: F, t6371: F, t119: F, t12351: F, t12419: F, t12420: F, t1343: F, t1354: F, t1363: F, t16321: F, t19871: F, t210: F, t3733: F, t3734: F, t3790: F, t3803: F, t5310: F, t54151: F, t54191: F, t54198: F, t56486: F, t56906: F, t56909: F, t56914: F, t56919: F, t56921: F, t6347: F, t820: F) -> F {
    let t56924 = t56923 * t1340;
    let t56927 = t12365 * t6417;
    let t56933 = t12283 * t19962;
    let t56935 = t12283 * t19882;
    let t56937 = t3866 * t19996;
    let t56946 = t40018 * t6371;
    let t56952 = -F::new(7.0) / F::new(288.0) * t56906 + F::new(595.0) / F::new(5184.0) * t54151 - F::new(35.0) / F::new(288.0) * t56909 + F::new(5.0) / F::new(192.0) * t16321 * t5310 + t3790 * t1343 * t820 * t56914 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t56919 + F::new(7.0) / F::new(1152.0) * t56921 - t56924 * t1354 / F::new(1536.0) - F::new(119.0) / F::new(13824.0) * t56927 - F::new(5.0) / F::new(768.0) * t3803 * t12419 * t19871 * t12420 + F::new(7.0) / F::new(2304.0) * t56933 - F::new(7.0) / F::new(576.0) * t56935 - F::new(35.0) / F::new(576.0) * t56937 - F::new(5.0) / F::new(128.0) * t1363 * t12351 * t820 * t6347 * t3734 + F::new(35.0) / F::new(96.0) * t54191 - F::new(119.0) / F::new(3456.0) * t54198 + F::new(35.0) / F::new(72.0) * t56946 + t3733 * t210 * t119 * t56486 / F::new(8.0);
    t56952
}
