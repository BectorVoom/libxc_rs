//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1210/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1210<F: Float>(t81688: F, t81716: F, t24269: F, t24278: F, t2679: F, t2684: F, t7101: F, t808: F, t812: F, t81656: F, t81661: F, t81667: F, t81670: F, t81675: F, t81691: F, t81695: F, t81697: F, t81702: F, t81704: F, t81709: F, t81713: F, t9958: F) -> F {
    let t84995 = F::cast_from(0.27415567780803773942e-2_f64) * t81688;
    let t85003 = F::cast_from(0.19739208802178717238e0_f64) * t81716;
    let t85007 = -t812 * t7101 * t9958 - F::cast_from(3.0_f64) * t812 * t24269 * t2679 + F::cast_from(0.9869604401089358619e-1_f64) * t81656 - F::cast_from(0.9869604401089358619e-1_f64) * t81661 + F::cast_from(3.0_f64) * t808 * t24278 - F::cast_from(0.49348022005446793095e-1_f64) * t81667 + F::cast_from(0.49348022005446793095e-1_f64) * t81670 - F::cast_from(0.16449340668482264365e-1_f64) * t81675 - t84995 + F::cast_from(0.24674011002723396548e-1_f64) * t81691 + F::cast_from(0.29608813203268075857e0_f64) * t81695 + F::cast_from(0.11514538467937585055e0_f64) * t81697 - F::cast_from(0.49348022005446793095e-1_f64) * t81702 + F::cast_from(0.11514538467937585055e0_f64) * t81704 - F::cast_from(0.49348022005446793095e-1_f64) * t81709 + F::cast_from(0.9869604401089358619e-1_f64) * t81713 + t85003 - F::cast_from(3.0_f64) * t812 * t24269 * t2684;
    t85007
}
