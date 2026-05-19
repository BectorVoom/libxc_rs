//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 927/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk927<F: Float>(t73909: F, t73912: F, t73922: F, t73938: F, t73940: F, t73944: F, t1970: F, t1971: F, t209: F, t2447: F, t476: F, t515: F) -> (F, F, F, F, F, F, F) {
    let t76768 = F::cast_from(0.2627895913935205078e-5_f64) * t73909;
    let t76769 = F::cast_from(0.2627895913935205078e-5_f64) * t73912;
    let t76771 = F::cast_from(0.16351352353374609375e-5_f64) * t73922;
    let t76779 = F::cast_from(0.20455996240684006296e-1_f64) * t73938;
    let t76780 = F::cast_from(0.40911992481368012592e-1_f64) * t73940;
    let t76781 = F::cast_from(0.20455996240684006296e-1_f64) * t73944;
    let t76786 = t1970 * t1971 * t515 * t2447 * t476 * t209;
    (t76768, t76769, t76771, t76779, t76780, t76781, t76786)
}
