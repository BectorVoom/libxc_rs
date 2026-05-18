//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1355/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1355<F: Float>(t225: F, t85754: F, t3545: F, t7372: F, t7378: F, t24698: F, t7327: F, t11148: F, t11504: F, t1186: F, t11888: F, t11889: F, t15022: F, t2148: F, t24589: F, t24784: F, t24788: F, t24806: F, t24829: F, t24833: F, t24859: F, t3477: F, t3624: F, t7283: F, t7363: F, t7373: F, t7377: F, t7381: F, t7386: F, t85836: F) -> F {
    let t85909 = t85754 * t225;
    let t85917 = t7372 * t3545;
    let t85918 = t85917 * t7378;
    let t85920 = t24698 * t7327;
    let t85933 = -F::new(3.0) * t3624 * t7386 * t15022 - F::new(6.0) * t11888 * t85836 * t11889 - F::new(0.24674011002723396548e-1) * t7283 * t3477 * t7381 - F::new(0.82246703342411321825e-2) * t7283 * t11504 * t2148 - F::new(0.8529287754027840782e-2) * t7283 * t85909 * t7363 * t11148 + F::new(0.16449340668482264365e-1) * t24589 * t24788 * t24859 - F::new(0.54831135561607547884e-2) * t85918 - F::new(0.24674011002723396548e-1) * t7373 * t85920 * t7377 - F::new(0.49348022005446793095e-1) * t7373 * t24833 * t24784 - F::new(0.24674011002723396548e-1) * t7373 * t24833 * t24806 - F::new(0.24674011002723396548e-1) * t7283 * t1186 * t24829;
    t85933
}
