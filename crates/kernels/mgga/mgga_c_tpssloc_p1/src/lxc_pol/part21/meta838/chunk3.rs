//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2993/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2993<F: Float>(t10231: F, t17157: F, t973: F, t17161: F, t17183: F, t2970: F, t17178: F, t17599: F, t17602: F, t17994: F, t2960: F, t43228: F, t50242: F, t50250: F, t50255: F, t50258: F, t50262: F, t59730: F, t59746: F, t977: F) -> F {
    let t62657 = t973 * t10231 * t17157;
    let t62660 = t973 * t10231 * t17161;
    let t62663 = t973 * t2970 * t17183;
    let t62666 = t973 * t10231 * t17178;
    let t62680 = F::new(4.0) / F::new(27.0) * t2960 * t17994 - F::new(4.0) / F::new(81.0) * t2960 * t17599 - F::new(14.0) / F::new(243.0) * t2960 * t17602 - t62657 / F::new(54.0) + t62660 / F::new(162.0) - t62663 / F::new(216.0) + t62666 / F::new(324.0) - t973 * t977 * t59730 / F::new(72.0) + t43228 / F::new(1296.0) - t50242 / F::new(108.0) - t50250 / F::new(216.0) + t50255 / F::new(384.0) + t50258 / F::new(3456.0) - t50262 / F::new(5184.0) + t973 * t977 * t59746 / F::new(48.0);
    t62680
}
