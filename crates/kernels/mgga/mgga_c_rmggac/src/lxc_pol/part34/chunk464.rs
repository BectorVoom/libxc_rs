//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 464/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk464<F: Float>(t352: F, t9523: F, t9001: F, t9009: F, t305: F, t5148: F, t8971: F, t8973: F, t8998: F, t9003: F, t9006: F, t9011: F, t9013: F, t9015: F, t9017: F, t9021: F, t9023: F, t9383: F) -> (F, F) {
    let t9577 = t9523 * t352;
    let t9583 = F::new(0.15965655602485078085e0) * t9001;
    let t9586 = F::new(0.23948483403727617128e0) * t9009;
    let t9593 = F::new(0.5987120850931904282e-1) * t8971 - F::new(0.5987120850931904282e-1) * t8973 - F::new(0.11974241701863808564e0) * t5148 * t9577 + F::new(0.59871208509319042821e-1) * t305 * t9383 + F::new(0.79828278012425390427e-1) * t8998 - t9583 + F::new(0.5987120850931904282e-1) * t9003 + F::new(0.5987120850931904282e-1) * t9006 + t9586 - F::new(0.17961362552795712846e0) * t9011 + F::new(0.35922725105591425692e0) * t9013 + F::new(0.8980681276397856423e-1) * t9015 - F::new(0.17961362552795712846e0) * t9017 - F::new(0.5987120850931904282e-1) * t9021 + F::new(0.8980681276397856423e-1) * t9023;
    (t9577, t9593)
}
