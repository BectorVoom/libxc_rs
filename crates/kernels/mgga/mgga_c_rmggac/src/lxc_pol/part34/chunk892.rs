//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 892/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk892<F: Float>(t71630: F, t75789: F, t71634: F, t15470: F, t2604: F, t699: F, t8700: F, t903: F, t75794: F, t3225: F, t39953: F, t75800: F, t75803: F, t1627: F, t3204: F, t71633: F, t71639: F, t75771: F, t75774: F, t75780: F, t75792: F, t75797: F) -> (F,) {
    let t78287 = 0.18183107769496894486e-1 * t71630;
    let t78288 = 0.19709219354514038085e-5 * t75789;
    let t78290 = 0.99317399751028291929e-5 * t71634;
    let t78294 = t2604 * t15470;
    let t78295 = 0.2993560425465952141e-1 * t78294;
    let t78297 = t903 * t699 * t8700;
    let t78298 = 0.44903406381989282115e-1 * t78297;
    let t78299 = 0.79828278012425390427e-1 * t75794;
    let t78300 = t39953 * t3225;
    let t78301 = 0.34093327067806677161e-2 * t78300;
    let t78303 = 0.2627895913935205078e-5 * t75800;
    let t78304 = 0.2627895913935205078e-5 * t75803;
    let t78305 = 0.18637685463734316849e-1 * t75771 - 0.46594213659335792122e-1 * t75774 - 0.93188427318671584245e-2 * t75780 - t78287 + t78288 - t71633 + 0.87596530464506835935e-6 * t75792 + t78290 + t71639 + 0.17961362552795712846e0 * t903 * t3204 * t1627 - t78295 - t78298 - t78299 - t78301 - 0.17519306092901367187e-5 * t75797 + t78303 - t78304;
    (t78305,)
}
