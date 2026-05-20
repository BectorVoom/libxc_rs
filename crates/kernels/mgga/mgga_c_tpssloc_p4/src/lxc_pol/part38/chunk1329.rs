//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1329/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1329<F: Float>(t110626: F, t1266: F, t1268: F, t12725: F, t12734: F, t16503: F, t1849: F, t19456: F, t2180: F, t2181: F, t2183: F, t26114: F, t29934: F, t29947: F, t30180: F, t30189: F, t30201: F, t30203: F, t4034: F, t510: F, t5107: F, t5113: F, t55934: F, t55962: F, t574: F, t652: F, t7676: F, t8143: F, t8144: F, t8148: F, t8150: F, t8237: F, t90381: F) -> F {
    let t110826 = -F::new(4.0) * t4034 * t30203 - F::new(4.0) * t652 * t1266 * t30180 + F::new(4.0) * t55934 * t2183 + F::new(4.0) * t12725 * t8148 + F::new(2.0) * t1268 * t2180 * t16503 - F::new(4.0) * t55934 * t2181 + F::new(2.0) * t1268 * t29934 * t1849 + F::new(4.0) * t26114 * t8150 + F::new(4.0) * t12734 * t8237 + F::new(4.0) * t5113 * t30201 + F::new(2.0) * t1268 * t110626 * t574 + F::new(4.0) * t7676 * t29947 - F::new(2.0) * t652 * t510 * t110626 - F::new(4.0) * t4034 * t30189 + F::new(2.0) * t90381 * t2183 + F::new(2.0) * t55962 * t2183 + F::new(4.0) * t19456 * t8148 - F::new(4.0) * t12725 * t8144 - F::new(4.0) * t652 * t5107 * t8143 + F::new(4.0) * t12725 * t8150;
    t110826
}
