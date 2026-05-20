//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1953/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1953<F: Float>(t1458: F, t8103: F, t1459: F, t1849: F, t2114: F, t2167: F, t27863: F, t28027: F, t28029: F, t28032: F, t28034: F, t28036: F, t28038: F, t28040: F, t28042: F, t28047: F, t28240: F, t29486: F, t29497: F, t510: F, t5460: F, t5494: F, t574: F, t6287: F, t6468: F, t652: F, t7266: F, t8107: F) -> (F, F) {
    let t29501 = t8103 * t1458;
    let t29506 = -F::new(4.0) * t1459 * t27863 + F::new(2.0) * t1849 * t8107 - t2114 * t6287 + t2167 * t6468 - t29486 * t510 + t29497 * t574 - F::new(4.0) * t29501 * t652 - F::new(4.0) * t5460 * t7266 - F::new(2.0) * t5494 * t7266 - t28027 - t28029 - t28032 - t28034 - t28036 - t28038 - t28040 - t28042 - t28047 + t28240;
    (t29501, t29506)
}
