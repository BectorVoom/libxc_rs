//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1059/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1059<F: Float>(t12344: F, t1336: F, t1369: F, t12215: F, t12317: F, t12320: F, t12323: F, t12325: F, t12330: F, t12331: F, t12335: F, t12336: F, t12340: F, t3783: F, t3876: F, t559: F) -> (F, F) {
    let t12345 = t1336 * t12344;
    let t12346 = t12345 * t1369;
    let t12348 = -F::new(7.0) / F::new(16.0) * t12317 - t12215 * t12320 / F::new(4.0) - F::new(7.0) / F::new(1536.0) * t12323 + F::new(119.0) / F::new(4608.0) * t12325 - t12330 + t12331 * t559 / F::new(3072.0) - t12335 - t12336 * t1369 / F::new(256.0) + F::new(7.0) / F::new(192.0) * t12340 - t3783 * t3876 / F::new(256.0) - F::new(119.0) / F::new(1152.0) * t12346;
    (t12345, t12348)
}
