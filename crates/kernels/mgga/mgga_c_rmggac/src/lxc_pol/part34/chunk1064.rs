//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1064/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1064<F: Float>(t71630: F, t75789: F, t71634: F, t15470: F, t2604: F, t699: F, t8700: F, t903: F, t75794: F, t3225: F, t39953: F, t75800: F) -> (F, F, F, F, F, F, F, F) {
    let t78287 = F::new(0.18183107769496894486e-1) * t71630;
    let t78288 = F::new(0.19709219354514038085e-5) * t75789;
    let t78290 = F::new(0.99317399751028291929e-5) * t71634;
    let t78294 = t2604 * t15470;
    let t78295 = F::new(0.2993560425465952141e-1) * t78294;
    let t78297 = t903 * t699 * t8700;
    let t78298 = F::new(0.44903406381989282115e-1) * t78297;
    let t78299 = F::new(0.79828278012425390427e-1) * t75794;
    let t78300 = t39953 * t3225;
    let t78301 = F::new(0.34093327067806677161e-2) * t78300;
    let t78303 = F::new(0.2627895913935205078e-5) * t75800;
    (t78287, t78288, t78290, t78295, t78298, t78299, t78301, t78303)
}
