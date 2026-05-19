//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 975/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk975<F: Float>(t75469: F, t75473: F, t75477: F, t75480: F, t75484: F, t75508: F, t75513: F, t75517: F, t75522: F, t75533: F, t75536: F, t75561: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77631 = F::cast_from(0.5107751987195740728e-4_f64) * t75469;
    let t77633 = F::cast_from(0.7661627980793611092e-4_f64) * t75473;
    let t77634 = F::cast_from(0.5107751987195740728e-4_f64) * t75477;
    let t77635 = F::cast_from(0.2553875993597870364e-4_f64) * t75480;
    let t77636 = F::cast_from(0.43368970657079495308e-4_f64) * t75484;
    let t77641 = F::cast_from(0.86737941314158990619e-4_f64) * t75508;
    let t77642 = F::cast_from(0.81300399444200075499e-3_f64) * t75513;
    let t77643 = F::cast_from(0.54549323308490683461e-1_f64) * t75517;
    let t77646 = F::cast_from(0.9197635698773217773e-5_f64) * t75522;
    let t77653 = F::cast_from(0.2627895913935205078e-5_f64) * t75533;
    let t77654 = F::cast_from(0.7883687741805615234e-5_f64) * t75536;
    let t77658 = F::cast_from(0.10511583655740820312e-4_f64) * t75561;
    (t77631, t77633, t77634, t77635, t77636, t77641, t77642, t77643, t77646, t77653, t77654, t77658)
}
