//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 862/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk862<F: Float>(t13937: F, t75374: F, t13940: F, t75416: F, t69742: F, t10570: F, t14077: F, t15309: F, t2046: F, t2049: F, t2339: F, t3167: F, t39953: F) -> (F, F, F, F, F, F) {
    let t75423 = t13937 * t75374;
    let t75425 = t13940 * t75416;
    let t75440 = F::new(0.59590439850616975158e-4) * t69742;
    let t75443 = t10570 * t14077 * t15309;
    let t75446 = t2046 * t2049 * t2339;
    let t75448 = t39953 * t3167;
    (t75423, t75425, t75440, t75443, t75446, t75448)
}
