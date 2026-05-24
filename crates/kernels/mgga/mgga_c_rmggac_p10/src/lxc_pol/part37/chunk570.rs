//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 570/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk570<F: Float>(t14935: F, t515: F, t235: F, t13825: F, t13833: F, t13837: F, t13842: F, t13844: F, t13854: F, t13864: F, t13890: F, t13893: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14936 = t515 * t14935;
    let t14937 = t235 * t14936;
    let t14938 = F::cast_from(0.19957069503106347607e-1_f64) * t14937;
    let t14939 = F::cast_from(0.76860658247009135562e-5_f64) * t13825;
    let t14940 = F::cast_from(0.93188427318671584242e-2_f64) * t13833;
    let t14941 = F::cast_from(0.15531404553111930707e-1_f64) * t13837;
    let t14942 = F::cast_from(0.31062809106223861414e-2_f64) * t13842;
    let t14944 = F::cast_from(0.31062809106223861414e-2_f64) * t13844;
    let t14945 = F::cast_from(0.58171619854173713844e-5_f64) * t13854;
    let t14946 = F::cast_from(0.17519306092901367187e-6_f64) * t13864;
    let t14947 = F::cast_from(0.35038612185802734374e-6_f64) * t13890;
    let t14950 = F::cast_from(0.52557918278704101561e-6_f64) * t13893;
    (t14936, t14938, t14939, t14940, t14941, t14942, t14944, t14945, t14946, t14947, t14950)
}
