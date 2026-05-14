//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 829/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk829<F: Float>(t75484: F, t75508: F, t75513: F, t75517: F, t75522: F, t75533: F, t75536: F, t75561: F, t75564: F, t75566: F, t75575: F, t75580: F, t75583: F, t75585: F, t75587: F, t14589: F, t8533: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77636 = 0.43368970657079495308e-4 * t75484;
    let t77641 = 0.86737941314158990619e-4 * t75508;
    let t77642 = 0.81300399444200075499e-3 * t75513;
    let t77643 = 0.54549323308490683461e-1 * t75517;
    let t77646 = 0.9197635698773217773e-5 * t75522;
    let t77653 = 0.2627895913935205078e-5 * t75533;
    let t77654 = 0.7883687741805615234e-5 * t75536;
    let t77658 = 0.10511583655740820312e-4 * t75561;
    let t77659 = 0.2627895913935205078e-5 * t75564;
    let t77660 = 0.2627895913935205078e-5 * t75566;
    let t77664 = 0.10248087766267884741e-3 * t75575;
    let t77665 = 0.38430329123504567781e-4 * t75580;
    let t77666 = 0.72042316457491791901e-3 * t75583;
    let t77669 = 0.1276937996798935182e-3 * t75585;
    let t77670 = 0.1915406995198402773e-3 * t75587;
    let t77671 = t14589 * t8533;
    (t77636, t77641, t77642, t77643, t77646, t77653, t77654, t77658, t77659, t77660, t77664, t77665, t77666, t77669, t77670, t77671)
}
