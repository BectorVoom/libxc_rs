//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 926/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk926<F: Float>(t73877: F, t73879: F, t73881: F, t73896: F, t3219: F, t38472: F, t1971: F, t2447: F, t495: F, t515: F, t7230: F, t73902: F) -> (F, F, F, F, F, F, F) {
    let t76752 = F::new(0.2553875993597870364e-4) * t73877;
    let t76753 = F::new(0.1702583995731913576e-4) * t73879;
    let t76755 = F::new(0.85129199786595678799e-5) * t73881;
    let t76757 = F::new(0.85129199786595678799e-5) * t73896;
    let t76758 = t38472 * t3219;
    let t76759 = F::new(0.42564599893297839398e-5) * t76758;
    let t76763 = t7230 * t1971 * t515 * t2447 * t495;
    let t76764 = F::new(0.53205749866622299248e-5) * t76763;
    let t76766 = F::new(0.19709219354514038085e-5) * t73902;
    (t76752, t76753, t76755, t76757, t76759, t76764, t76766)
}
