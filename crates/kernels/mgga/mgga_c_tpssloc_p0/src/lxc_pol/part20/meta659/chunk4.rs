//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2456/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2456<F: Float>(t10937: F, t13765: F, t3040: F, t607: F, t883: F, t1023: F, t10957: F, t10962: F, t14211: F, t14215: F, t3070: F, t3071: F, t42388: F, t42483: F, t42505: F, t43246: F, t43248: F, t43253: F, t43254: F, t43361: F, t4337: F, t4585: F, t4590: F, t4652: F, t48611: F, t48612: F, t49616: F, t49976: F) -> F {
    let t50272 = t10937 * t13765;
    let t50281 = t3040 * t883 * t607;
    let t50301 = t10962 * t4652 / F::new(1024.0) - t50272 / F::new(216.0) + t42483 * t48611 * t49616 * t1023 / F::new(1024.0) - t43246 / F::new(288.0) - t43248 / F::new(648.0) - t43253 + t42388 * t3071 * t48612 * t50281 / F::new(256.0) - t43361 * t3071 * t14211 * t50281 / F::new(256.0) - t42505 * t14215 / F::new(72.0) + t3070 * t3071 * t4337 * t49976 / F::new(256.0) - F::new(19.0) / F::new(432.0) * t10957 * t4585 + F::new(95.0) / F::new(2592.0) * t10957 * t4590 - t43254 / F::new(288.0);
    t50301
}
