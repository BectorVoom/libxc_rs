//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 797/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk797<F: Float>(t73881: F, t73896: F, t3219: F, t38472: F, t1971: F, t2447: F, t495: F, t515: F, t7230: F, t73902: F, t73909: F, t73912: F, t73922: F, t73887: F, t73891: F, t73899: F, t73906: F, t73920: F, t73924: F, t73926: F, t73929: F, t73931: F) -> (F,) {
    let t76755 = 0.85129199786595678799e-5 * t73881;
    let t76757 = 0.85129199786595678799e-5 * t73896;
    let t76758 = t38472 * t3219;
    let t76759 = 0.42564599893297839398e-5 * t76758;
    let t76763 = t7230 * t1971 * t515 * t2447 * t495;
    let t76764 = 0.53205749866622299248e-5 * t76763;
    let t76766 = 0.19709219354514038085e-5 * t73902;
    let t76768 = 0.2627895913935205078e-5 * t73909;
    let t76769 = 0.2627895913935205078e-5 * t73912;
    let t76771 = 0.16351352353374609375e-5 * t73922;
    let t76776 = -t76755 + t73887 - 0.17519306092901367188e-6 * t73891 + t76757 - t76759 + t76764 - 0.87596530464506835935e-6 * t73899 - t76766 - 0.87596530464506835935e-6 * t73906 + t76768 - t76769 - 0.35038612185802734376e-6 * t73920 - t76771 - 0.81756761766873046873e-5 * t73924 + 0.29085809927086856923e-4 * t73926 - 0.17519306092901367187e-5 * t73929 + 0.87596530464506835935e-6 * t73931;
    (t76776,)
}
