//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1357/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1357<F: Float>(t131: F, t467: F, t50: F, t82510: F, t10469: F, t461: F, t11715: F, t11721: F, t3032: F, t3502: F, t3508: F, t1090: F, t11498: F, t11882: F, t2148: F, t24858: F, t3248: F, t3471: F, t7283: F, t7362: F, t7381: F, t85941: F, t85943: F, t85945: F, t85947: F, t85952: F, t85955: F) -> (F, F, F) {
    let t85963 = t50 * t82510 * t131 * t467;
    let t85964 = t461 * t10469;
    let t85965 = t85964 * t11715;
    let t85966 = t3032 * t11721;
    let t85971 = t85964 * t3502;
    let t85972 = t3032 * t3508;
    let t85977 = -F::new(0.82246703342411321825e-2) * t7283 * t11498 * t2148 - F::new(0.24674011002723396548e-1) * t7283 * t3471 * t7381 - F::new(0.54831135561607547884e-2) * t85941 - F::new(0.27415567780803773942e-2) * t85943 - F::new(0.54831135561607547883e-2) * t85945 - F::new(0.82246703342411321826e-2) * t7283 * t7362 * t85947 * t1090 + F::new(0.18277045187202515961e-2) * t85952 + F::new(0.82246703342411321826e-2) * t85955 - F::new(0.16449340668482264365e-1) * t7283 * t7362 * t24858 * t3248 + F::new(0.49348022005446793095e-1) * t85963 * t85965 * t11882 * t85966 - F::new(0.49348022005446793095e-1) * t85963 * t85971 * t11882 * t85972;
    (t85963, t85964, t85977)
}
