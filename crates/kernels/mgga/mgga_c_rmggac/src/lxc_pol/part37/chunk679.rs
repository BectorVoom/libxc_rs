//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 679/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk679<F: Float>(t14116: F, t14125: F, t8421: F, t15361: F, t498: F, t14236: F, t2067: F, t68471: F, t321: F, t69629: F, t333: F, t69588: F, t1981: F, t676: F, t687: F, t8512: F) -> (F, F, F, F, F) {
    let t73984 = t14116 * t14125 * t8421;
    let t73986 = t15361 * t498;
    let t73989 = t14236 * t68471 * t2067 * t73986;
    let t73991 = t15361 * t321;
    let t73994 = t14236 * t69629 * t2067 * t73991;
    let t73996 = t15361 * t333;
    let t73999 = t14236 * t69588 * t2067 * t73996;
    let t74003 = t8512 * t1981 * t676 * t687;
    (t73984, t73989, t73994, t73999, t74003)
}
