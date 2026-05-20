//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2723/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2723<F: Float>(t12300: F, t6422: F, t12365: F, t1358: F, t19836: F, t12250: F, t6387: F, t12429: F, t16101: F, t16215: F, t16217: F, t16225: F, t16233: F, t16305: F, t16311: F, t16312: F, t16401: F, t1825: F, t19735: F, t19886: F, t19890: F, t221: F, t3803: F, t5240: F, t5246: F, t53973: F, t54063: F, t54555: F, t54557: F, t54561: F, t54567: F, t56560: F, t57086: F, t6388: F, t6394: F) -> F {
    let t57308 = t12300 * t6422;
    let t57310 = t12365 * t6422;
    let t57324 = t19836 * t1358;
    let t57342 = t6387 * t12250;
    let t57351 = F::new(7.0) / F::new(2304.0) * t57308 - F::new(119.0) / F::new(13824.0) * t57310 - F::new(119.0) / F::new(3456.0) * t54555 + F::new(7.0) / F::new(2304.0) * t54557 - F::new(7.0) / F::new(1152.0) * t54561 + F::new(7.0) / F::new(384.0) * t54567 - t16401 * t19890 / F::new(96.0) - t5246 * t16305 * t19735 * t16225 / F::new(96.0) - F::new(5.0) / F::new(64.0) * t5240 * t16217 - F::new(7.0) / F::new(2304.0) * t57324 - t5246 * t16305 * t16311 * t57086 / F::new(96.0) + t12429 * t19886 / F::new(192.0) + t3803 * t16305 * t53973 * t6394 / F::new(192.0) - t16101 * t221 * t56560 + F::new(5.0) / F::new(64.0) * t3803 * t54063 * t1825 * t16215 + t16233 * t16305 * t57342 * t16312 / F::new(64.0) - t5246 * t16305 * t6388 * t16312 / F::new(64.0);
    t57351
}
