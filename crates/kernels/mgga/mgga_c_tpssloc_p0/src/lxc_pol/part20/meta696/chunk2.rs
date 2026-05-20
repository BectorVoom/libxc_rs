//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2656/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2656<F: Float>(t16155: F, t3866: F, t1827: F, t40123: F, t1824: F, t3850: F, t16060: F, t3802: F, t1799: F, t1340: F, t53909: F, t12255: F, t12305: F, t12336: F, t1307: F, t1354: F, t1363: F, t16018: F, t16150: F, t16217: F, t16224: F, t16225: F, t16305: F, t16306: F, t3783: F, t3803: F, t3807: F, t3809: F, t3851: F, t3870: F, t5240: F, t5246: F, t5248: F, t5249: F, t5310: F, t54013: F, t820: F) -> F {
    let t54138 = t3866 * t16155;
    let t54151 = t40123 * t1827;
    let t54153 = t1824 * t3850;
    let t54162 = t16060 * t3802;
    let t54165 = t1799 * t3850;
    let t54178 = t53909 * t1340;
    let t54183 = -F::new(35.0) / F::new(384.0) * t54138 + F::new(5.0) / F::new(128.0) * t3783 * t16150 + F::new(5.0) / F::new(256.0) * t1363 * t3870 * t820 * t16018 * t1307 + F::new(5.0) / F::new(256.0) * t12336 * t5310 + F::new(5.0) / F::new(256.0) * t5240 * t12305 + F::new(595.0) / F::new(10368.0) * t54151 + t3803 * t16305 * t54153 * t3807 / F::new(256.0) + F::new(7.0) / F::new(1536.0) * t5246 * t5248 * t5249 * t12255 + t54162 * t3809 / F::new(128.0) - F::new(5.0) / F::new(256.0) * t3803 * t16224 * t54165 * t3807 + t3803 * t16305 * t16225 * t3851 / F::new(256.0) - t3803 * t54013 * t16306 * t3851 / F::new(1024.0) - t54178 * t1354 / F::new(1024.0) - F::new(15.0) / F::new(128.0) * t3783 * t16217;
    t54183
}
