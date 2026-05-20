//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2700/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2700<F: Float>(t12283: F, t19976: F, t19886: F, t16257: F, t16261: F, t16305: F, t16306: F, t16311: F, t19876: F, t19956: F, t19984: F, t3803: F, t3805: F, t3856: F, t5246: F, t5248: F, t5259: F, t5287: F, t54013: F, t54086: F, t54088: F, t54090: F, t54092: F, t54114: F, t54116: F, t54118: F, t54162: F, t54165: F, t54258: F, t6394: F) -> F {
    let t56837 = t12283 * t19976;
    let t56853 = t12283 * t19886;
    let t56866 = -F::new(7.0) / F::new(288.0) * t54086 - F::new(7.0) / F::new(576.0) * t54088 + F::new(7.0) / F::new(1152.0) * t54090 + F::new(7.0) / F::new(2304.0) * t54092 - t3803 * t5248 * t19956 * t3856 / F::new(3072.0) + F::new(7.0) / F::new(2304.0) * t56837 - t5246 * t16305 * t16311 * t54165 / F::new(192.0) + F::new(7.0) / F::new(1152.0) * t54114 + t19876 * t16257 / F::new(384.0) + t19876 * t16261 / F::new(768.0) + t3803 * t16305 * t54258 * t6394 / F::new(384.0) - F::new(7.0) / F::new(576.0) * t54116 - F::new(7.0) / F::new(288.0) * t56853 - t3803 * t54013 * t16306 * t5287 / F::new(768.0) - F::new(7.0) / F::new(576.0) * t54118 + t54162 * t5259 / F::new(192.0) + t3803 * t3805 * t19984 * t3856 / F::new(768.0);
    t56866
}
