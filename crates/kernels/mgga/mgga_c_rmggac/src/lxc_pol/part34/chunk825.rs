//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 825/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk825<F: Float>(t14314: F, t570: F, t262: F, t8620: F, t1652: F, t3080: F, t41063: F, t739: F, t7577: F, t7778: F, t8946: F, t903: F) -> (F, F, F, F, F, F, F, F) {
    let t74811 = t14314 * t570;
    let t74812 = t262 * t74811;
    let t74813 = t8620 * t74812;
    let t74815 = t3080 * t1652;
    let t74816 = t262 * t74815;
    let t74817 = t8620 * t74816;
    let t74824 = F::cast_from(0.5987120850931904282e-1_f64) * t739 * t7577 * t41063;
    let t74829 = t903 * t7778 * t8946;
    (t74811, t74812, t74813, t74815, t74816, t74817, t74824, t74829)
}
