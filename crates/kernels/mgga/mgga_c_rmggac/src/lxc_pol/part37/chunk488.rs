//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 488/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk488<F: Float>(t13917: F, t13940: F, t3102: F, t7348: F, t2060: F, t7799: F, t1550: F, t7879: F, t903: F, t352: F, t664: F) -> (F, F, F, F, F, F, F) {
    let t13941 = t13940 * t13917;
    let t13943 = t3102 * t7348;
    let t13949 = t2060 * t7799;
    let t13950 = t1550 * t13949;
    let t13951 = F::new(0.5987120850931904282e-1) * t13950;
    let t13954 = t2060 * t7879;
    let t13955 = t903 * t13954;
    let t13956 = F::new(0.8980681276397856423e-1) * t13955;
    let t13957 = t664 * t352;
    (t13941, t13943, t13949, t13951, t13954, t13956, t13957)
}
