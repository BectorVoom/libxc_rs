//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2453/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2453<F: Float>(t13961: F, t3109: F, t10263: F, t10321: F, t10403: F, t10408: F, t14122: F, t1616: F, t3070: F, t3071: F, t3132: F, t42505: F, t42541: F, t43200: F, t43206: F, t43214: F, t43219: F, t43221: F, t43226: F, t43241: F, t4337: F, t4347: F, t4609: F) -> F {
    let t50229 = t3109 * t13961;
    let t50237 = t42541 * t14122 / F::new(768.0) - t43200 / F::new(3456.0) + t10403 * t3071 * t4347 * t3132 / F::new(768.0) - t43206 / F::new(1152.0) + t43214 / F::new(648.0) + t43219 / F::new(3456.0) + t43221 / F::new(432.0) + t43226 / F::new(2304.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10408 * t4337 * t43241 + F::new(11.0) / F::new(108.0) * t10263 * t4609 - t50229 / F::new(144.0) - t42505 * t14122 / F::new(144.0) + t3070 * t3071 * t1616 * t10321 / F::new(4608.0);
    t50237
}
