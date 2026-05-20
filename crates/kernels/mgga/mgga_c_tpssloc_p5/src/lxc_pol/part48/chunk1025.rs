//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1025/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1025<F: Float>(t115238: F, t115245: F, t115249: F, t115251: F, t115254: F, t115256: F, t115261: F, t115265: F, t115271: F, t2039: F, t2165: F, t23917: F, t23953: F, t23958: F, t24176: F, t24442: F, t24924: F, t31832: F, t32365: F, t4034: F, t652: F, t7056: F, t7171: F, t7266: F, t7408: F, t8690: F) -> F {
    let t117590 = -F::new(2.0) * t2039 * t24924 * t652 - F::new(2.0) * t2165 * t23917 * t652 - F::new(4.0) * t652 * t7056 * t7408 + F::new(3.0) * t23953 * t8690 + F::new(6.0) * t23958 * t8690 + F::new(6.0) * t24176 * t8690 - F::new(2.0) * t24442 * t7266 + F::new(6.0) * t31832 * t7171 - F::new(4.0) * t32365 * t4034 + t115238 + t115245 - t115249 - t115251 - t115254 - t115256 - t115261 + t115265 - t115271;
    t117590
}
