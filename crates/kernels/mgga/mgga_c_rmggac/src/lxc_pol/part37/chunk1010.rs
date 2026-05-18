//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1010/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1010<F: Float>(t2329: F, t72109: F, t2344: F, t71229: F, t14581: F, t8526: F, t75758: F, t71630: F, t75789: F, t71634: F, t15470: F, t2604: F) -> (F, F, F, F, F, F, F, F) {
    let t78274 = t72109 * t2329;
    let t78275 = F::new(0.13637330827122670864e-1) * t78274;
    let t78276 = t71229 * t2344;
    let t78277 = F::new(0.10227998120342003148e-1) * t78276;
    let t78278 = t14581 * t8526;
    let t78279 = F::new(0.10227998120342003148e-1) * t78278;
    let t78280 = F::new(0.14967802127329760705e-1) * t75758;
    let t78287 = F::new(0.18183107769496894486e-1) * t71630;
    let t78288 = F::new(0.19709219354514038085e-5) * t75789;
    let t78290 = F::new(0.99317399751028291929e-5) * t71634;
    let t78294 = t2604 * t15470;
    (t78275, t78277, t78279, t78280, t78287, t78288, t78290, t78294)
}
