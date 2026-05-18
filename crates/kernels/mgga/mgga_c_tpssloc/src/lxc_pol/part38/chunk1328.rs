//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1328/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1328<F: Float>(t12823: F, t1774: F, t19456: F, t2181: F, t2183: F, t2314: F, t26114: F, t26179: F, t29890: F, t29934: F, t30181: F, t30186: F, t30189: F, t30201: F, t30203: F, t30209: F, t30211: F, t4028: F, t4034: F, t45632: F, t5113: F, t55962: F, t652: F, t8124: F, t8148: F, t8221: F, t8231: F, t8235: F, t8237: F, t90370: F, t91753: F, t9348: F) -> F {
    let t110778 = F::new(4.0) * t2314 * t30181 - F::new(2.0) * t12823 * t8221 - F::new(4.0) * t4034 * t30209 + F::new(4.0) * t90370 * t2183 + F::new(4.0) * t26114 * t8148 + F::new(4.0) * t5113 * t30211 - F::new(2.0) * t55962 * t2181 - F::new(4.0) * t19456 * t8124 + F::new(2.0) * t45632 * t2183 - F::new(4.0) * t2314 * t30189 + F::new(4.0) * t2314 * t30201 + F::new(4.0) * t5113 * t30186 + F::new(2.0) * t9348 * t8235 + F::new(2.0) * t9348 * t8237 - F::new(2.0) * t91753 * t2181 - F::new(4.0) * t26179 * t8124 - F::new(2.0) * t652 * t1774 * t29934 - F::new(2.0) * t9348 * t8231 - F::new(4.0) * t4028 * t29890 - F::new(4.0) * t2314 * t30203;
    t110778
}
