//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2679/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2679<F: Float>(t1799: F, t5286: F, t16224: F, t16305: F, t1825: F, t19919: F, t19924: F, t3803: F, t40006: F, t40060: F, t54063: F, t57007: F, t57009: F, t57011: F, t57019: F, t57022: F, t57041: F, t57057: F, t57071: F, t57073: F) -> (F, F) {
    let t74677 = t1799 * t5286;
    let t74682 = F::new(455.0) / F::new(648.0) * t40006 + F::new(15.0) / F::new(128.0) * t3803 * t54063 * t1825 * t19919 - F::new(5.0) / F::new(128.0) * t3803 * t16224 * t1825 * t19924 - F::new(35.0) / F::new(384.0) * t57007 + F::new(7.0) / F::new(384.0) * t57009 + F::new(595.0) / F::new(1152.0) * t57011 - F::new(119.0) / F::new(1152.0) * t57019 + F::new(7.0) / F::new(384.0) * t57022 + F::new(595.0) / F::new(2592.0) * t40060 + F::new(119.0) / F::new(2304.0) * t57041 + F::new(7.0) / F::new(1536.0) * t57057 - F::new(7.0) / F::new(768.0) * t57071 - F::new(119.0) / F::new(2304.0) * t57073 + t3803 * t16305 * t1825 * t74677 / F::new(128.0);
    (t74677, t74682)
}
