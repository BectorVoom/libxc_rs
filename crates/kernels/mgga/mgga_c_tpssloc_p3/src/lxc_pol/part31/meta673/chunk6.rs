//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2030/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2030<F: Float>(t102558: F, t102580: F, t102597: F, t102614: F, t102629: F, t102765: F, t102790: F, t102822: F, t1375: F, t1378: F, t27068: F, t27115: F, t29372: F, t3758: F, t5215: F, t5354: F, t84423: F, t90706: F, t93461: F, t93467: F, t97529: F, t97537: F, t97548: F) -> F {
    let t102828 = F::cast_from(0.15352717957250113407e0_f64) * t97529 + t84423 - F::new(2.0) * t5215 * t27115 + t93461 + F::cast_from(0.76763589786250567037e-1_f64) * t97537 + t90706 + t93467 - F::cast_from(0.76763589786250567037e-1_f64) * t97548 - F::new(2.0) * t27068 * t5354 + F::new(2.0) * t3758 * t29372 - t1375 * t1378 * (t102558 + t102580 + t102597 + t102614 + t102629 + t102765 + t102790 + t102822);
    t102828
}
