//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 755/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk755<F: Float>(t1716: F, t8010: F, t27382: F, t2130: F, t46: F, t1932: F, t2133: F, t7573: F, t8027: F, t1737: F, t2136: F, t24681: F, t24704: F, t27578: F, t27592: F, t27599: F, t27609: F, t27614: F, t6203: F, t6211: F, t7345: F) -> (F, F, F) {
    let t29554 = t1716 * t8010;
    let t29557 = t1716 * t27382;
    let t29560 = t2130 * t46;
    let t29561 = 1.0 / t29560;
    let t29562 = t29561 * t1932;
    let t29563 = t29562 * t2133;
    let t29569 = t8027 * t7573;
    let t29580 = -t24681 + 0.72670960969452703541e-2 * t29563 * t2136 + t27578 / 1152.0 - t7345 * t6211 / 1152.0 + 0.16149102437656156342e-2 * t29569 * t2136 - t24704 - t27592 / 216.0 - 0.20186378047070195428e-3 * t27609 - t27599 * t1737 / 144.0 + t27614 * t1737 / 768.0 + 5.0 / 6912.0 * t7345 * t6203;
    (t29554, t29557, t29580)
}
