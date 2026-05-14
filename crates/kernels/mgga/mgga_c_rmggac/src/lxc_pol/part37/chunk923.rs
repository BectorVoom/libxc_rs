//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 923/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk923<F: Float>(t14936: F, t623: F, t71633: F, t71639: F, t73420: F, t75771: F, t75774: F, t75780: F, t75792: F, t75797: F, t78287: F, t78288: F, t78290: F, t78295: F, t78298: F, t78299: F, t78301: F, t78303: F) -> (F,) {
    let t80327 = t73420 + 0.18637685463734316848e-1 * t75771 - 0.46594213659335792121e-1 * t75774 - 0.93188427318671584242e-2 * t75780 - 0.19957069503106347607e-1 * t623 * t14936 - t78287 + t78288 - t71633 + 0.87596530464506835932e-6 * t75792 + t78290 + t71639 - t78295 - t78298 - t78299 - t78301 - 0.17519306092901367186e-5 * t75797 + t78303;
    (t80327,)
}
