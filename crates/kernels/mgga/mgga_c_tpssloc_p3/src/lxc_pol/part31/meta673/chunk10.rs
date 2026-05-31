//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2034/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2034<F: Float>(t225: F, t29287: F, t16439: F, t19647: F, t19648: F, t20029: F, t2092: F, t24095: F, t26224: F, t26989: F, t29361: F, t3758: F, t5210: F, t56607: F, t568: F, t6461: F, t7194: F, t7199: F, t7918: F, t7937: F, t84705: F, t91548: F, t97766: F) -> (F, F) {
    let t102948 = t29287 * t225;
    let t102972 = -t3758 * t29361 + F::cast_from(0.6579736267392905746e-1_f64) * t91548 + F::cast_from(2.0_f64) * t5210 * t7918 * t568 - t24095 * t6461 - F::cast_from(0.6579736267392905746e-1_f64) * t97766 + F::cast_from(4.0_f64) * t7194 * t19648 - t84705 - F::cast_from(2.0_f64) * t56607 * t2092 - F::cast_from(12.0_f64) * t26224 * t26989 * t19647 + F::cast_from(4.0_f64) * t20029 * t7199 - F::cast_from(2.0_f64) * t16439 * t7937;
    (t102948, t102972)
}
