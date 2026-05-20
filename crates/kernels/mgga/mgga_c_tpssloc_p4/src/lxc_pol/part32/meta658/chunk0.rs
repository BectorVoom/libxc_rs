//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2087/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2087<F: Float>(t90659: F, t90663: F, t90837: F, t90868: F, t90900: F, t90980: F, t90993: F, t91000: F, t91149: F, t91167: F, t91305: F, t91312: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93445 = F::cast_from(0.12793931631041761173e0_f64) * t90659;
    let t93446 = F::cast_from(0.16449340668482264365e-1_f64) * t90663;
    let t93517 = F::cast_from(0.10417915756705434098e0_f64) * t90837;
    let t93538 = F::cast_from(0.12793931631041761173e0_f64) * t90868;
    let t93563 = F::cast_from(0.52089578783527170489e-1_f64) * t90900;
    let t93595 = F::cast_from(0.16449340668482264365e-1_f64) * t90980;
    let t93605 = F::cast_from(0.16449340668482264365e-1_f64) * t90993;
    let t93615 = F::cast_from(0.12793931631041761173e0_f64) * t91000;
    let t93650 = F::new(119.0) / F::new(864.0) * t91149;
    let t93656 = F::cast_from(0.22608743412718618878e-1_f64) * t91167;
    let t93721 = F::new(119.0) / F::new(3456.0) * t91305;
    let t93723 = F::cast_from(0.10541775202358879834e-2_f64) * t91312;
    (t93445, t93446, t93517, t93538, t93563, t93595, t93605, t93615, t93650, t93656, t93721, t93723)
}
