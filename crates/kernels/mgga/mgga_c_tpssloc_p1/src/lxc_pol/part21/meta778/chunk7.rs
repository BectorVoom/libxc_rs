//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2697/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2697<F: Float>(t16398: F, t19966: F, t5259: F, t53945: F, t119: F, t12419: F, t1315: F, t16148: F, t16233: F, t16305: F, t16314: F, t16401: F, t19873: F, t19876: F, t19979: F, t19984: F, t20468: F, t210: F, t3793: F, t3805: F, t39936: F, t39948: F, t39950: F, t40168: F, t5246: F, t5301: F, t53921: F, t53927: F, t53929: F, t53946: F, t53965: F, t53973: F, t54013: F, t54014: F, t54258: F, t54614: F, t56275: F) -> F {
    let t56693 = t16398 * t19966;
    let t56710 = t53945 * t5259;
    let t56729 = -F::new(7.0) / F::new(12.0) * t53921 + F::new(35.0) / F::new(18.0) * t53927 + F::new(7.0) / F::new(6.0) * t53929 + t39936 - F::new(7.0) / F::new(1152.0) * t56693 - t19876 * t16314 / F::new(96.0) - t5246 * t16305 * t54258 * t20468 / F::new(64.0) - F::new(7.0) / F::new(288.0) * t53946 - F::new(119.0) / F::new(6912.0) * t39948 - F::new(119.0) / F::new(13824.0) * t39950 + t16401 * t19873 / F::new(256.0) - F::new(5.0) / F::new(32.0) * t54614 * t40168 * t5301 * t16148 - F::new(7.0) / F::new(288.0) * t56710 + F::new(5.0) / F::new(384.0) * t5246 * t12419 * t19979 * t3793 - t5246 * t3805 * t19984 * t3793 / F::new(384.0) - t16233 * t54013 * t54014 * t53973 / F::new(128.0) - t1315 * t210 * t119 * t56275 / F::new(48.0) + F::new(35.0) / F::new(288.0) * t53965;
    t56729
}
