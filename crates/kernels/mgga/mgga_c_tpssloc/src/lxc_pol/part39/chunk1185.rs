//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1185/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1185<F: Float>(t110972: F, t111017: F, t111168: F, t111213: F, t112: F, t30349: F, t110376: F, t110926: F, t111143: F, t1401: F, t1458: F, t16524: F, t16535: F, t16538: F, t2199: F, t30071: F, t30109: F, t30112: F, t30128: F, t30315: F, t3938: F, t3941: F, t4072: F, t45560: F, t5371: F, t55341: F, t55405: F, t577: F, t66940: F, t671: F, t8189: F, t8273: F, t8294: F) -> (F, F) {
    let t111215 = t110972 + t111017 + t111168 + t111213;
    let t111226 = t30349 * t112;
    let t111243 = 27.0 * t3938 * t30315 + 0.135e2 * t55341 * t2199 + 54.0 * t30112 * t16538 + 27.0 * t110926 * t1458 + 27.0 * t16524 * t30128 + 0.45e1 * t111215 * t577 + 27.0 * t3941 * t30071 * t1458 + 54.0 * t3941 * t8189 * t4072 + 27.0 * t45560 * t8294 + 27.0 * t111226 * t671 + 27.0 * t30109 * t4072 + 0.135e2 * t5371 * t30071 + 27.0 * t16535 * t8273 + 54.0 * t66940 * t8294 + 27.0 * t55405 * t2199 + 0.135e2 * t110376 * t1458 + 0.135e2 * t1401 * t111143;
    (t111215, t111243)
}
