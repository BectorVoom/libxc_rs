//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 565/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk565<F: Float>(t14639: F, t675: F, t2186: F, t3219: F, t14144: F, t1356: F, t14441: F, t14156: F, t14171: F, t14175: F, t14186: F, t14190: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14640 = t675 * t14639;
    let t14641 = F::new(0.42564599893297839398e-5) * t14640;
    let t14642 = t2186 * t3219;
    let t14649 = F::new(0.14967802127329760705e-1) * t14144;
    let t14650 = t1356 * t14441;
    let t14651 = F::new(0.39914139006212695214e-1) * t14650;
    let t14653 = F::new(0.10227998120342003148e-1) * t14156;
    let t14655 = F::new(0.44903406381989282115e-1) * t14171;
    let t14656 = F::new(0.14967802127329760705e-1) * t14175;
    let t14659 = F::new(0.85129199786595678799e-5) * t14186;
    let t14660 = F::new(0.2553875993597870364e-4) * t14190;
    (t14641, t14642, t14649, t14651, t14653, t14655, t14656, t14659, t14660)
}
