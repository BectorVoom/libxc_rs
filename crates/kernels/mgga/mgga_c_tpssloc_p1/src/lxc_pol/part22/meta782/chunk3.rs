//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2675/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675<F: Float>(t12286: F, t12351: F, t1307: F, t1341: F, t1343: F, t1363: F, t1799: F, t19631: F, t19921: F, t19926: F, t20416: F, t20497: F, t20556: F, t20565: F, t3778: F, t3783: F, t3870: F, t5187: F, t5240: F, t56776: F, t56779: F, t56795: F, t56797: F, t6330: F, t6347: F, t74564: F, t820: F) -> F {
    let t74569 = F::new(7.0) / F::new(192.0) * t56776 + F::new(7.0) / F::new(192.0) * t56779 - F::new(119.0) / F::new(576.0) * t56795 + F::new(7.0) / F::new(384.0) * t56797 + t12286 * t20497 / F::new(512.0) + F::new(5.0) / F::new(768.0) * t1363 * t3870 * t820 * t20416 * t1307 - F::new(15.0) / F::new(128.0) * t1363 * t12351 * t820 * t6330 * t5187 + F::new(5.0) / F::new(256.0) * t3783 * t20565 + F::new(5.0) / F::new(256.0) * t1363 * t3870 * t820 * t5187 * t6347 + F::new(5.0) / F::new(256.0) * t1363 * t3870 * t820 * t1799 * t19631 - F::new(15.0) / F::new(128.0) * t5240 * t19921 + F::new(5.0) / F::new(128.0) * t5240 * t19926 - t3778 * t20556 / F::new(3072.0) - t1341 * t1343 * t820 * t74564 / F::new(3072.0);
    t74569
}
