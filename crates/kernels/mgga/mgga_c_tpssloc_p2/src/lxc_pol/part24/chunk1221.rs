//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1221/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1221<F: Float>(t12461: F, t3698: F, t2019: F, t1983: F, t113: F, t1976: F, t22594: F, t22599: F, t22600: F, t22605: F, t22608: F, t22610: F, t22612: F, t22614: F, t22616: F, t22618: F, t22619: F, t22950: F, t2312: F, t2364: F, t23829: F, t23833: F, t23835: F, t23837: F, t23855: F, t510: F, t574: F, t6517: F, t652: F) -> (F, F, F) {
    let t23857 = t12461 * t3698;
    let t23858 = t2019 * t23857;
    let t23860 = F::new(2.0) * t1983 * t23858;
    let t23861 = -t113 * t23829 - t1976 * t2312 - F::new(2.0) * t22600 * t510 - F::new(4.0) * t22619 * t652 - F::new(2.0) * t2364 * t6517 + t23855 * t574 + t22594 + t22599 + t22605 + t22608 - t22610 - t22612 - t22614 - t22616 - t22618 + t22950 - t23833 - t23835 + t23837 + t23860;
    (t23857, t23858, t23861)
}
