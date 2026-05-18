//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 540/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk540<F: Float>(t14327: F, t793: F, t3065: F, t6444: F, t13988: F, t5259: F, t13992: F, t4669: F, t14078: F, t2500: F, t14102: F, t3075: F) -> (F, F, F, F, F, F) {
    let t14328 = t793 * t14327;
    let t14330 = t6444 * t3065;
    let t14333 = F::new(0.5987120850931904282e-1) * t5259 * t13988;
    let t14335 = F::new(0.8980681276397856423e-1) * t4669 * t13992;
    let t14336 = t2500 * t14078;
    let t14338 = t3075 * t14102;
    (t14328, t14330, t14333, t14335, t14336, t14338)
}
