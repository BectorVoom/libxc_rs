//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 749/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk749<F: Float>(t345: F, t4988: F, t947: F, t242: F, t4977: F, t2762: F, t4826: F, t4830: F, t970: F, t4834: F, t2652: F, t2670: F, t2722: F, t2731: F, t2740: F, t3917: F, t3942: F, t3970: F, t4966: F, t4970: F, t4974: F, t4980: F, t4985: F, t925: F, t946: F, t967: F) -> (F, F, F, F, F, F, F, F) {
    let t4989 = t4988 * t345;
    let t4990 = t947 * t4989;
    let t4991 = t242 * t4990;
    let t4994 = t4977 * t345;
    let t4995 = t947 * t4994;
    let t4996 = t242 * t4995;
    let t5000 = t2762 * t4826;
    let t5001 = t242 * t5000;
    let t5004 = t970 * t4830;
    let t5005 = t242 * t5004;
    let t5008 = t970 * t4834;
    let t5009 = t242 * t5008;
    let t5012 = -t2670 + t3917 / F::new(432.0) + t925 * t4966 / F::new(216.0) - t925 * t4970 / F::new(144.0) + t925 * t4974 / F::new(288.0) + t2722 * t4980 / F::new(1536.0) + t3942 / F::new(2304.0) + t2740 * t4985 / F::new(2304.0) + t946 * t4991 / F::new(3072.0) - t2731 * t4996 / F::new(3072.0) - t2652 + t3970 / F::new(3456.0) + F::new(5.0) / F::new(13824.0) * t967 * t5001 - t967 * t5005 / F::new(2304.0) + t967 * t5009 / F::new(4608.0);
    (t4989, t4991, t4994, t4996, t5001, t5005, t5009, t5012)
}
