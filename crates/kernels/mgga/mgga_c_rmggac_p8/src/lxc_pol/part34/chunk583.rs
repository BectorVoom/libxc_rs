//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 583/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk583<F: Float>(t15078: F, t1550: F, t291: F, t8465: F, t13823: F, t3080: F, t570: F, t5148: F, t551: F, t5259: F, t558: F, t4669: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15079 = t1550 * t15078;
    let t15081 = t8465 * t291;
    let t15082 = t13823 * t15081;
    let t15084 = t3080 * t570;
    let t15086 = F::cast_from(0.5987120850931904282e-1_f64) * t5148 * t15084;
    let t15087 = t3080 * t551;
    let t15089 = F::cast_from(0.5987120850931904282e-1_f64) * t5259 * t15087;
    let t15090 = t3080 * t558;
    let t15092 = F::cast_from(0.8980681276397856423e-1_f64) * t4669 * t15090;
    (t15079, t15081, t15082, t15084, t15086, t15087, t15089, t15090, t15092)
}
