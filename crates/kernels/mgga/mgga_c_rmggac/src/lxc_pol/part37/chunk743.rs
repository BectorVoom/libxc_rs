//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 743/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk743<F: Float>(t13897: F, t75411: F, t15098: F, t30526: F, t1326: F, t75399: F, t13916: F, t13928: F, t1612: F, t11704: F, t13931: F, t13937: F, t75374: F, t13940: F, t69742: F, t10570: F, t14077: F, t15309: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t75412 = t75411 * t13897;
    let t75414 = t30526 * t15098;
    let t75416 = t1326 * t75399;
    let t75417 = t13916 * t75416;
    let t75419 = t13928 * t1612;
    let t75421 = t13931 * t11704;
    let t75423 = t13937 * t75374;
    let t75425 = t13940 * t75416;
    let t75440 = 0.59590439850616975158e-4 * t69742;
    let t75443 = t10570 * t14077 * t15309;
    (t75412, t75414, t75416, t75417, t75419, t75421, t75423, t75425, t75440, t75443)
}
