//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2100/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2100<F: Float>(t1404: F, t7758: F, t1395: F, t7774: F, t86586: F, t86870: F, t86911: F, t86916: F, t86955: F, t86991: F, t87068: F, t87080: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t91818 = F::cast_from(2.0_f64) * t7758 * t1404;
    let t91824 = F::cast_from(2.0_f64) * t1395 * t7774;
    let t92121 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t86586;
    let t92383 = F::cast_from(0.10417915756705434098e0_f64) * t86870;
    let t92402 = F::cast_from(0.52089578783527170489e-1_f64) * t86911;
    let t92406 = F::cast_from(0.3289868133696452873e-1_f64) * t86916;
    let t92432 = F::cast_from(0.12793931631041761173e0_f64) * t86955;
    let t92458 = F::cast_from(0.12793931631041761173e0_f64) * t86991;
    let t92492 = F::cast_from(0.52089578783527170489e-1_f64) * t87068;
    let t92497 = F::cast_from(0.12793931631041761173e0_f64) * t87080;
    (t91818, t91824, t92121, t92383, t92402, t92406, t92432, t92458, t92492, t92497)
}
