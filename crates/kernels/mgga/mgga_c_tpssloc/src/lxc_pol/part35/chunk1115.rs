//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1115/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1115<F: Float>(t5: F, t29484: F, t112: F, t2113: F, t5456: F, t1458: F, t27863: F, t28001: F, t28004: F, t28006: F, t28009: F, t28011: F, t28019: F, t5493: F, t7266: F, t8103: F, t1459: F, t1849: F, t2114: F, t2167: F, t28027: F, t28029: F, t28032: F, t28034: F, t28036: F, t28038: F, t28040: F, t28042: F, t28047: F, t28240: F, t510: F, t5460: F, t5494: F, t574: F, t6287: F, t6468: F, t652: F, t8107: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t29485 = piecewise3(t8, 0.0, t29484);
    let t29486 = t29485 * t112;
    let t29493 = t2113 * t5456;
    let t29497 = 4.0 * t1458 * t27863 + 2.0 * t5493 * t7266 + t28001 + t28004 + t28006 + t28009 + t28011 + t28019 + t29486 + 2.0 * t29493;
    let t29501 = t8103 * t1458;
    let t29506 = -4.0 * t1459 * t27863 + 2.0 * t1849 * t8107 - t2114 * t6287 + t2167 * t6468 - t29486 * t510 + t29497 * t574 - 4.0 * t29501 * t652 - 4.0 * t5460 * t7266 - 2.0 * t5494 * t7266 - t28027 - t28029 - t28032 - t28034 - t28036 - t28038 - t28040 - t28042 - t28047 + t28240;
    (t29485, t29486, t29493, t29497, t29501, t29506)
}
